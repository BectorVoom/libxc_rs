//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 686/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk686<F: Float>(t2095: F, t831: F, t1420: F, t2481: F, t1426: F, t2480: F, t439: F, t444: F, t5961: F, t442: F, t2485: F, t2484: F, t3279: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6238 = t831 * t2095 / F::new(15.0);
    let t6240 = t1420 * t2481 / F::new(45.0);
    let t6241 = t1426 * t2480;
    let t6243 = t439 * t6241 / F::new(45.0);
    let t6244 = t444 * t5961;
    let t6245 = t442 * t6244;
    let t6247 = t439 * t6245 / F::new(45.0);
    let t6249 = t1420 * t2485 / F::new(27.0);
    let t6250 = t3279 * t2484;
    (t6238, t6240, t6241, t6243, t6244, t6245, t6247, t6249, t6250)
}
