//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 908/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk908(t1047: f64, t1656: f64, t25498: f64, t25539: f64, t27448: f64, t27450: f64, t27460: f64, t27462: f64, t27464: f64, t27467: f64, t27496: f64, t27518: f64, t27541: f64, t375: f64, t4803: f64, t4808: f64, t7132: f64) -> f64 {
    let t27543 = 0.28582678745379824648e-3_f64 * t27448 + 0.42874018118069736972e-3_f64 * t27450 * t1047 - 0.57165357490759649296e-3_f64 * t7132 * t4803 + 0.47637797908966374413e-3_f64 * t7132 * t4808 - 0.28582678745379824648e-3_f64 * t25498 - t25539 * t1656 / 108.0_f64 + t27460 / 864.0_f64 + 0.28582678745379824648e-3_f64 * t27462 - 0.22866142996303859718e-2_f64 * t27464 * t375 + 0.42874018118069736972e-3_f64 * t27467 * t375 + t27496 + t27518 + t27541;
    t27543
}
