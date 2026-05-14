//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 812/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk812<F: Float>(t2679: F, t3726: F, t9796: F, t12240: F, t2617: F, t7810: F, t47143: F, t825: F, t969: F, t2365: F, t39149: F, t7390: F, t41231: F, t41244: F, t39118: F, t959: F) -> (F, F, F, F, F, F, F) {
    let t47212 = t9796 * t3726 * t2679;
    let t47215 = t7810 * t12240 * t2617;
    let t47344 = t825 * t969 * t47143;
    let t47347 = t7390 * t2365 * t39149;
    let t47377 = 0.63904876589867916128e-1 * t41231;
    let t47378 = 0.63904876589867916128e-1 * t41244;
    let t47379 = t39118 * t959;
    (t47212, t47215, t47344, t47347, t47377, t47378, t47379)
}
