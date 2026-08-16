//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1905;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta562(t13826: f64, t7271: f64, t13923: f64, t7264: f64, t14036: f64, t25997: f64, t13946: f64, t26028: f64, t13941: f64, t94423: f64, t14005: f64, t13834: f64, t27940: f64, t13841: f64, t5706: f64, t94429: f64, t1941: f64, t9817: f64, t48662: f64, t5651: f64, t7028: f64, t9736: f64, t13985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98176, t98178, t98180, t98182, t98185, t98187, t98189) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1905(t13826, t7271, t13923, t7264, t14036, t25997, t13946, t26028, t13941, t94423, t14005, t13834, t27940);
        let (t98191, t98193, t98197, t98200, t98202) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1906(t13841, t26028, t5706, t94429, t1941, t9817, t48662, t5651, t7028, t9736, t13985, t94423);
    (t98176, t98178, t98180, t98182, t98185, t98187, t98189, t98191, t98193, t98197, t98200, t98202)
}
