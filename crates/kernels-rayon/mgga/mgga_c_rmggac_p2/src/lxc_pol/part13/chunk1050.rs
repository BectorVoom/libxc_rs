//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1050/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1050(t39233: f64, t39250: f64, t39252: f64, t39255: f64, t39264: f64, t2868: f64, t39231: f64, t39238: f64, t39243: f64, t39248: f64, t39258: f64, t39262: f64, t39266: f64, t39271: f64, t39275: f64, t39278: f64, t5055: f64, t8212: f64, t8215: f64) -> f64 {
    let t42886 = 0.39726959900411316772e-4_f64 * t39233;
    let t42890 = 0.11918087970123395032e-3_f64 * t39250;
    let t42891 = 0.11918087970123395032e-3_f64 * t39252;
    let t42892 = 0.60975299583150056624e-3_f64 * t39255;
    let t42899 = 0.39726959900411316772e-4_f64 * t39264;
    let t42904 = -0.638468998399467591e-4_f64 * t39231 - t42886 - 0.1702583995731913576e-4_f64 * t39238 - 0.10215503974391481456e-3_f64 * t39243 - 0.5107751987195740728e-4_f64 * t39248 - t42890 + t42891 - t42892 + 0.35922725105591425692e0_f64 * t5055 * t8212 + 0.23948483403727617128e0_f64 * t2868 * t8215 - 0.10909864661698136692e0_f64 * t39258 + 0.5107751987195740728e-4_f64 * t39262 + t42899 + 0.10215503974391481456e-3_f64 * t39266 - 0.2553875993597870364e-4_f64 * t39271 - 0.7661627980793611092e-4_f64 * t39275 + 0.212822999466489197e-4_f64 * t39278;
    t42904
}
