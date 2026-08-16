//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta466(t3930: f64, t6846: f64, t221: f64, t4019: f64, t6862: f64, t10001: f64, t6800: f64, t72: f64, t757: f64, t1317: f64, t6801: f64, t13599: f64, t21901: f64, t21905: f64, t21933: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t22179, t22182, t22183, t22187, t22189, t22190) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1710(t3930, t6846, t221, t4019, t6862, t10001, t6800, t72, t757, t1317, t6801, t13599, t21901, t21905, t21933, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22179, t22182, t22183, t22187, t22189, t22190)
}
