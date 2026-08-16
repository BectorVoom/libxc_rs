//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1078/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1078(t4147: f64, t8594: f64, t8598: f64, t9593: f64, t1450: f64, t211: f64, t9644: f64, t11006: f64, t256: f64, t2410: f64, t10308: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36970 = t4147 * t8594;
    let t37110 = t9593 * t8598;
    let t37956 = t8594 * t1450;
    let t37972 = t8598 * t4147;
    let t39643 = 1.0_f64 / t9644 / t211;
    let t41077 = 1.0_f64 / t11006 / t256;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0_f64 / t41153;
    let t45963 = t599 * t10308;
    (t36970, t37110, t37956, t37972, t39643, t41077, t41154, t45963)
}
