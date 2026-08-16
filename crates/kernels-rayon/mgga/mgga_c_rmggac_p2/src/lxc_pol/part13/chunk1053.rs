//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1053/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1053(t39405: f64, t35002: f64, t35053: f64, t35056: f64, t35058: f64, t39388: f64, t39390: f64, t39394: f64, t39396: f64, t39401: f64, t39403: f64, t39418: f64, t39420: f64, t39423: f64, t39425: f64, t39433: f64, t39435: f64, t6355: f64, t8063: f64) -> f64 {
    let t42954 = 0.39726959900411316772e-4_f64 * t39405;
    let t42964 = -0.11974241701863808564e0_f64 * t6355 * t8063 + 0.72042316457491791901e-3_f64 * t35002 + 0.29810146462873361016e-2_f64 * t39388 - 0.5107751987195740728e-4_f64 * t39390 - 0.1702583995731913576e-4_f64 * t39394 - 0.85129199786595678799e-5_f64 * t39396 - 0.85129199786595678799e-5_f64 * t39401 - 0.1702583995731913576e-4_f64 * t39403 + t42954 - 0.81300399444200075499e-3_f64 * t35053 - 0.162600798888400151e-2_f64 * t35056 - 0.2553875993597870364e-4_f64 * t39418 + 0.5107751987195740728e-4_f64 * t39420 - 0.23836175940246790063e-3_f64 * t35058 + 0.35922725105591425692e0_f64 * t39423 + 0.11974241701863808564e0_f64 * t39425 - 0.3192344991997337955e-4_f64 * t39433 - 0.5107751987195740728e-4_f64 * t39435;
    t42964
}
