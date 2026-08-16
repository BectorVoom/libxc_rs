//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1872;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta554(t5697: f64, t94429: f64, t5701: f64, t27928: f64, t9775: f64, t13775: f64, t25986: f64, t2661: f64, t25978: f64, t5614: f64, t5622: f64, t94443: f64, t13769: f64, t240: f64, t7269: f64, t13760: f64, t25972: f64, t5609: f64, t7028: f64, t9845: f64, t1889: f64, t94545: f64, t13846: f64, t13877: f64, t7021: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98128, t98130, t98141, t98144, t98146, t98148) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1872(t5697, t94429, t5701, t27928, t9775, t13775, t25986, t2661, t25978, t5614, t5622, t94443);
        let (t98152, t98156, t98161, t98165, t98168) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1873(t13769, t240, t2661, t7269, t13760, t25972, t5609, t7028, t9845, t1889, t94545, t13846, t13877, t7021);
    (t98128, t98130, t98141, t98144, t98146, t98148, t98152, t98156, t98161, t98165, t98168)
}
