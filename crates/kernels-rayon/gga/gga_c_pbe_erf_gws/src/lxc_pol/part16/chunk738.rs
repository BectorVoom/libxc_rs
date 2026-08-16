//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 738/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk738(t1289: f64, t387: f64, t13: f64, t1292: f64, t30: f64, t4510: f64, t2704: f64, t2718: f64, t4518: f64, t4521: f64, t4524: f64, t4529: f64, t4531: f64, t4533: f64) -> (f64, f64) {
    let t4658 = 1.0_f64 / t1289 / t387;
    let t4659 = t13 * t4658;
    let t4661 = 1.0_f64 / t1292 / t30;
    let t4662 = t4510 * t4661;
    let t4663 = t4659 * t4662;
    let t4664 = 0.51725014705706168417e3_f64 * t4663;
    let t4673 = -0.47063e1_f64 * t4518 + 0.31375333333333333334e1_f64 * t4521 - 0.36604555555555555556e1_f64 * t4524 - 0.16068111111111111111e1_f64 * t2704 + 0.28051666666666666666e0_f64 * t4529 - 0.56103333333333333332e0_f64 * t4531 - 0.6545388888888888889e0_f64 * t4533 - 0.46308888888888888888e0_f64 * t2718;
    (t4664, t4673)
}
