//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 541/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk541<F: Float>(t2147: F, t7508: F, t649: F, t866: F, t27: F, t2145: F, t645: F, t798: F, t3928: F, t2060: F, t4048: F, t1550: F, t4905: F, t903: F, t665: F, t2024: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7509 = t7508 * t2147;
    let t7510 = 0.68186654135613354322e-2 * t7509;
    let t7511 = t649 * t866;
    let t7512 = t27 * t7511;
    let t7513 = t2145 * t7512;
    let t7514 = 0.34093327067806677161e-2 * t7513;
    let t7518 = t645 * t798;
    let t7519 = t3928 * t7518;
    let t7520 = 0.17961362552795712846e0 * t7519;
    let t7521 = t2060 * t4048;
    let t7522 = t1550 * t7521;
    let t7523 = 0.5987120850931904282e-1 * t7522;
    let t7524 = t2060 * t4905;
    let t7525 = t903 * t7524;
    let t7526 = 0.8980681276397856423e-1 * t7525;
    let t7527 = t665 * t798;
    let t7528 = t903 * t7527;
    let t7529 = 0.35922725105591425692e0 * t7528;
    let t7530 = t2024 * t4048;
    (t7510, t7512, t7514, t7518, t7520, t7521, t7523, t7524, t7526, t7527, t7529, t7530)
}
