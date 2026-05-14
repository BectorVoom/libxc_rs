//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 733/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk733<F: Float>(t1008: F, t2246: F, t6: F, t1007: F, t195: F, t287: F, t362: F, t357: F, t355: F, t2320: F, t993: F, t241: F, t2427: F, t847: F, t2471: F, t261: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7324 = t1008 * t2246 * t6;
    let t7325 = t1007 * t7324;
    let t7328 = t195 * t287;
    let t7329 = t7328 * t362;
    let t7330 = t357 * t7329;
    let t7332 = 5.0 / 27.0 * t355 * t7330;
    let t7335 = t2320 * t993;
    let t7337 = t241 * t2427;
    let t7339 = 0.17544670192365612213e1 * t7337 * t847;
    let t7341 = 1.0 / t2471 / t261;
    (t7324, t7325, t7328, t7329, t7330, t7332, t7335, t7337, t7339, t7341)
}
