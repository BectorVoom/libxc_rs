//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 864/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk864(t38946: f64, t38968: f64, t38986: f64, t38998: f64, t39023: f64, t39025: f64, t39031: f64, t39233: f64, t39250: f64, t39252: f64, t39255: f64, t39264: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42788 = 0.1454648621559751559e0_f64 * t38946;
    let t42794 = 0.49658699875514145965e-4_f64 * t38968;
    let t42800 = 0.11918087970123395032e-3_f64 * t38986;
    let t42806 = 0.11918087970123395032e-3_f64 * t38998;
    let t42820 = 0.36366215538993788974e-1_f64 * t39023;
    let t42821 = 0.10909864661698136692e0_f64 * t39025;
    let t42823 = 0.10909864661698136692e0_f64 * t39031;
    let t42886 = 0.39726959900411316772e-4_f64 * t39233;
    let t42890 = 0.11918087970123395032e-3_f64 * t39250;
    let t42891 = 0.11918087970123395032e-3_f64 * t39252;
    let t42892 = 0.60975299583150056624e-3_f64 * t39255;
    let t42899 = 0.39726959900411316772e-4_f64 * t39264;
    (t42788, t42794, t42800, t42806, t42820, t42821, t42823, t42886, t42890, t42891, t42892, t42899)
}
