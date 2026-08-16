//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 670/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk670(t1329: f64, t779: f64, t238: f64, t242: f64, t226: f64, t3309: f64, t2167: f64, t2203: f64, t2216: f64, t2218: f64, t3300: f64, t3311: f64, t3325: f64, t3330: f64, t3336: f64, t3338: f64, t3342: f64) -> (f64, f64, f64, f64, f64) {
    let t3344 = t779 * t1329;
    let t3346 = t238 * t242 * t3344;
    let t3348 = t226 * t3309;
    let t3350 = t238 * t242 * t3348;
    let t3352 = -0.9494625e0_f64 * t3325 + 0.1898925e1_f64 * t3330 + t2203 - 0.29896666666666666667e0_f64 * t2167 - 0.29896666666666666667e0_f64 * t3300 + 0.8969e0_f64 * t3311 + 0.15358125e0_f64 * t3336 + 0.3071625e0_f64 * t3338 + t2216 - 0.16431333333333333333e0_f64 * t2218 - 0.16431333333333333333e0_f64 * t3342 + 0.24647e0_f64 * t3346 + 0.24647e0_f64 * t3350;
    (t3344, t3346, t3348, t3350, t3352)
}
