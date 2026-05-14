//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 919/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk919<F: Float>(t39405: F, t35002: F, t35053: F, t35056: F, t35058: F, t39388: F, t39390: F, t39394: F, t39396: F, t39401: F, t39403: F, t39418: F, t39420: F, t39423: F, t39425: F, t39433: F, t39435: F, t6355: F, t8063: F) -> (F,) {
    let t42954 = 0.39726959900411316772e-4 * t39405;
    let t42964 = -0.11974241701863808564e0 * t6355 * t8063 + 0.72042316457491791901e-3 * t35002 + 0.29810146462873361016e-2 * t39388 - 0.5107751987195740728e-4 * t39390 - 0.1702583995731913576e-4 * t39394 - 0.85129199786595678799e-5 * t39396 - 0.85129199786595678799e-5 * t39401 - 0.1702583995731913576e-4 * t39403 + t42954 - 0.81300399444200075499e-3 * t35053 - 0.162600798888400151e-2 * t35056 - 0.2553875993597870364e-4 * t39418 + 0.5107751987195740728e-4 * t39420 - 0.23836175940246790063e-3 * t35058 + 0.35922725105591425692e0 * t39423 + 0.11974241701863808564e0 * t39425 - 0.3192344991997337955e-4 * t39433 - 0.5107751987195740728e-4 * t39435;
    (t42964,)
}
