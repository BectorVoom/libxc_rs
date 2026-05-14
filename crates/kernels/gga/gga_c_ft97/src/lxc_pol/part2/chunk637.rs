//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 637/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk637<F: Float>(t11008: F, t7954: F, t92: F, t11059: F, t1642: F, t11013: F, t3051: F, t11034: F, t11050: F, t378: F, t11003: F, t10998: F, t355: F, t358: F, t26: F, t2999: F) -> (F, F, F, F, F, F, F, F) {
    let t11179 = t7954 * t11008;
    let t11180 = t92 * t11179;
    let t11182 = t1642 * t11059;
    let t11183 = t92 * t11182;
    let t11185 = t1642 * t11013;
    let t11186 = t3051 * t11185;
    let t11188 = t1642 * t11034;
    let t11189 = t92 * t11188;
    let t11191 = t378 * t11050;
    let t11192 = t92 * t11191;
    let t11194 = t378 * t11003;
    let t11195 = t3051 * t11194;
    let t11197 = t378 * t10998;
    let t11198 = t92 * t11197;
    let t11200 = t355 * t358;
    let t11202 = t26 * t2999 * t11200;
    (t11180, t11183, t11186, t11189, t11192, t11195, t11198, t11202)
}
