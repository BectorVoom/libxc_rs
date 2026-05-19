//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 670/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk670<F: Float>(t1329: F, t779: F, t238: F, t242: F, t226: F, t3309: F, t2167: F, t2203: F, t2216: F, t2218: F, t3300: F, t3311: F, t3325: F, t3330: F, t3336: F, t3338: F, t3342: F) -> (F, F, F, F, F) {
    let t3344 = t779 * t1329;
    let t3346 = t238 * t242 * t3344;
    let t3348 = t226 * t3309;
    let t3350 = t238 * t242 * t3348;
    let t3352 = -F::new(0.9494625e0) * t3325 + F::new(0.1898925e1) * t3330 + t2203 - F::cast_from(0.29896666666666666667e0_f64) * t2167 - F::cast_from(0.29896666666666666667e0_f64) * t3300 + F::new(0.8969e0) * t3311 + F::new(0.15358125e0) * t3336 + F::new(0.3071625e0) * t3338 + t2216 - F::cast_from(0.16431333333333333333e0_f64) * t2218 - F::cast_from(0.16431333333333333333e0_f64) * t3342 + F::new(0.24647e0) * t3346 + F::new(0.24647e0) * t3350;
    (t3344, t3346, t3348, t3350, t3352)
}
