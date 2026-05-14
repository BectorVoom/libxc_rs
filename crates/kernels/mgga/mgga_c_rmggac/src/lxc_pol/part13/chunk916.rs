//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 916/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk916<F: Float>(t39233: F, t39250: F, t39252: F, t39255: F, t39264: F, t2868: F, t39231: F, t39238: F, t39243: F, t39248: F, t39258: F, t39262: F, t39266: F, t39271: F, t39275: F, t39278: F, t5055: F, t8212: F, t8215: F) -> (F,) {
    let t42886 = 0.39726959900411316772e-4 * t39233;
    let t42890 = 0.11918087970123395032e-3 * t39250;
    let t42891 = 0.11918087970123395032e-3 * t39252;
    let t42892 = 0.60975299583150056624e-3 * t39255;
    let t42899 = 0.39726959900411316772e-4 * t39264;
    let t42904 = -0.638468998399467591e-4 * t39231 - t42886 - 0.1702583995731913576e-4 * t39238 - 0.10215503974391481456e-3 * t39243 - 0.5107751987195740728e-4 * t39248 - t42890 + t42891 - t42892 + 0.35922725105591425692e0 * t5055 * t8212 + 0.23948483403727617128e0 * t2868 * t8215 - 0.10909864661698136692e0 * t39258 + 0.5107751987195740728e-4 * t39262 + t42899 + 0.10215503974391481456e-3 * t39266 - 0.2553875993597870364e-4 * t39271 - 0.7661627980793611092e-4 * t39275 + 0.212822999466489197e-4 * t39278;
    (t42904,)
}
