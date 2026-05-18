//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1057/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1057<F: Float>(t7508: F, t9803: F, t2145: F, t27: F, t6463: F, t649: F, t7494: F, t9807: F, t2025: F, t32556: F, t41582: F, t41585: F, t41605: F, t41614: F, t47340: F, t47345: F, t47347: F, t47349: F, t47351: F, t47353: F, t47355: F, t47357: F, t47359: F) -> F {
    let t47361 = t7508 * t9803;
    let t47365 = t2145 * t27 * t649 * t6463;
    let t47367 = t7494 * t9807;
    let t47369 = t41582 + F::new(0.8980681276397856423e-1) * t47340 + F::new(0.39914139006212695214e-1) * t32556 * t2025 - F::new(0.59590439850616975157e-4) * t41585 + F::new(0.85129199786595678796e-5) * t47345 - F::new(0.25538759935978703639e-4) * t47347 + F::new(0.25538759935978703639e-4) * t47349 + F::new(0.17025839957319135759e-4) * t47351 - F::new(0.85129199786595678796e-5) * t47353 + F::new(0.51077519871957407276e-4) * t47355 - F::new(0.76616279807936110914e-4) * t47357 - F::new(0.59590439850616975155e-4) * t47359 - F::new(0.34093327067806677161e-2) * t47361 - F::new(0.34093327067806677161e-2) * t47365 + F::new(0.10227998120342003148e-1) * t47367 - t41605 - t41614;
    t47369
}
