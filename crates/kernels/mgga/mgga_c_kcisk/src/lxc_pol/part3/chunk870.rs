//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 870/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk870<F: Float>(t1501: F, t4193: F, t4200: F, t4215: F, t13328: F, t484: F, t13331: F, t470: F, t487: F, t1496: F, t4312: F, t486: F, t380: F, t13777: F, t498: F, t493: F, sigma0: F) -> (F, F, F, F, F) {
    let t14361 = t1501 * t4193;
    let t14363 = t4215 * t4200;
    let t14364 = t484 * t13328;
    let t14365 = t14364 * sigma0;
    let t14366 = t470 * t13331;
    let t14367 = t487 * t14366;
    let t14368 = t14365 * t14367;
    let t14370 = t4312 * t1496;
    let t14371 = t486 * t14370;
    let t14374 = 1.0 / t470 / t380;
    let t14375 = t14374 * t13777;
    let t14376 = t498 * t14375;
    let t14377 = t493 * t14376;
    (t14361, t14363, t14368, t14371, t14377)
}
