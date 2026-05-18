//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1402/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1402<F: Float>(t1199: F, t9292: F, t2879: F, t2885: F, t1196: F, t9303: F, t481: F, t484: F, t9302: F, t2887: F, t2900: F, t26261: F) -> (F, F, F, F, F, F, F) {
    let t27918 = t9292 * t1199;
    let t27921 = t2879 * t2885;
    let t27926 = t1196 * t9303;
    let t27935 = t481 / t9302 / t484;
    let t27936 = t2887 * t2887;
    let t27942 = t2900 * t2900;
    let t27950 = F::new(0.75383950617283950617e4) * t26261;
    (t27918, t27921, t27926, t27935, t27936, t27942, t27950)
}
