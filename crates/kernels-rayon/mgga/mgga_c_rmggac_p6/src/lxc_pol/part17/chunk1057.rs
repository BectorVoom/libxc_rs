//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1057/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1057(t7508: f64, t9803: f64, t2145: f64, t27: f64, t6463: f64, t649: f64, t7494: f64, t9807: f64, t2025: f64, t32556: f64, t41582: f64, t41585: f64, t41605: f64, t41614: f64, t47340: f64, t47345: f64, t47347: f64, t47349: f64, t47351: f64, t47353: f64, t47355: f64, t47357: f64, t47359: f64) -> f64 {
    let t47361 = t7508 * t9803;
    let t47365 = t2145 * t27 * t649 * t6463;
    let t47367 = t7494 * t9807;
    let t47369 = t41582 + 0.8980681276397856423e-1_f64 * t47340 + 0.39914139006212695214e-1_f64 * t32556 * t2025 - 0.59590439850616975157e-4_f64 * t41585 + 0.85129199786595678796e-5_f64 * t47345 - 0.25538759935978703639e-4_f64 * t47347 + 0.25538759935978703639e-4_f64 * t47349 + 0.17025839957319135759e-4_f64 * t47351 - 0.85129199786595678796e-5_f64 * t47353 + 0.51077519871957407276e-4_f64 * t47355 - 0.76616279807936110914e-4_f64 * t47357 - 0.59590439850616975155e-4_f64 * t47359 - 0.34093327067806677161e-2_f64 * t47361 - 0.34093327067806677161e-2_f64 * t47365 + 0.10227998120342003148e-1_f64 * t47367 - t41605 - t41614;
    t47369
}
