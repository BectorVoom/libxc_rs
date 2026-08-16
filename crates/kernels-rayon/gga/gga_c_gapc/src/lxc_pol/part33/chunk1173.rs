//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1173/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1173(t1030: f64, t144: f64, t33521: f64, t34546: f64, t4052: f64, t34515: f64, t34517: f64, t34520: f64, t34522: f64, t34525: f64, t34528: f64, t34530: f64, t34533: f64, t34537: f64, t34539: f64) -> f64 {
    let t34547 = t1030 * t4052 * t33521 * t144 * t34546;
    let t34549 = 0.25301920572916666668e-5_f64 * t34515 + 0.12650960286458333334e-5_f64 * t34517 + 0.25301920572916666668e-5_f64 * t34520 + 0.12650960286458333334e-5_f64 * t34522 - 0.25301920572916666668e-5_f64 * t34525 - 0.24458523220486111112e-4_f64 * t34528 + 0.2845640240200497334e-7_f64 * t34530 + 0.34380927311705569432e-8_f64 * t34533 - 0.65555167711046006955e-8_f64 * t34537 + 0.70344136651018351214e-8_f64 * t34539 + 0.28199579487947481489e-8_f64 * t34547;
    t34549
}
