//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 954/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk954<F: Float>(t1102: F, t17529: F, t487: F, t870: F, t442: F, t8287: F, t444: F, t5239: F, t424: F, t1765: F, t496: F, t429: F, t8: F) -> (F, F, F, F, F, F) {
    let t17531 = F::cast_from(0.35089340384731224426e1_f64) * t1102 * t17529;
    let t17533 = t487 * t870;
    let t17534 = t442 * t442;
    let t17536 = t8287 * t17533 * t17534;
    let t17537 = t5239 * t444;
    let t17539 = F::new(1.0) / t424 / t17537;
    let t17542 = t1765 * t496;
    let t17543 = t17539 * t8 * t429 * t17542;
    (t17531, t17534, t17536, t17539, t17542, t17543)
}
