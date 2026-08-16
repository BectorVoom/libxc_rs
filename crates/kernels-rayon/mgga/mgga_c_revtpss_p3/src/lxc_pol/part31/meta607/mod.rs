//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2045;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta607(t1889: f64, t94545: f64, t13846: f64, t13877: f64, t7021: f64, t5665: f64, t94497: f64, t14036: f64, t25997: f64, t13941: f64, t94423: f64, t14005: f64, t5706: f64, t94429: f64, t1941: f64, t9817: f64, t5651: f64, t7028: f64, t9736: f64, t13985: f64, t13878: f64, t25972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98165, t98169, t98174, t98181, t98186, t98187) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2045(t1889, t94545, t13846, t13877, t7021, t5665, t94497, t14036, t25997, t13941, t94423, t14005);
        let (t98188, t98194, t98196, t98200, t98203, t98206) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2046(t98187, t5706, t94429, t1941, t9817, t5651, t7028, t9736, t13985, t94423, t13878, t25972);
    (t98165, t98169, t98174, t98181, t98186, t98188, t98194, t98196, t98200, t98203, t98206)
}
