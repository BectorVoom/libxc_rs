//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 813/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk813<F: Float>(t39123: F, t959: F, t13847: F, t2684: F, t7354: F, t41295: F, t41299: F, t41312: F, t41316: F, t12252: F, t2628: F, t13892: F, t5676: F, t12161: F, t2033: F, t2365: F, t2610: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47381 = t39123 * t959;
    let t47389 = t2684 * t7354 * t13847;
    let t47402 = 0.63904876589867916128e-1 * t41295;
    let t47403 = 0.63904876589867916128e-1 * t41299;
    let t47405 = 0.63904876589867916128e-1 * t41312;
    let t47406 = 0.63904876589867916128e-1 * t41316;
    let t47450 = t12252 * t2628;
    let t47488 = t5676 * t13892;
    let t47492 = t2033 * t2365 * t2610 * t12161;
    (t47381, t47389, t47402, t47403, t47405, t47406, t47450, t47488, t47492)
}
