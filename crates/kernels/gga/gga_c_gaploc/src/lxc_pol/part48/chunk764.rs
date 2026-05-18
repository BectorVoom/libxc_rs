//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 764/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk764<F: Float>(t1352: F, t3517: F, t2754: F, t986: F, t6508: F, t11279: F, t161: F, t11218: F, t158: F, t20368: F, t35845: F, t203: F) -> (F, F, F, F, F, F) {
    let t35908 = t3517 * t1352;
    let t35912 = t986 * t2754;
    let t35913 = t6508 * t35912;
    let t35918 = t11279 * t161;
    let t35951 = t158 * t11218;
    let t35959 = t20368 * t35845;
    let t36117 = t203 * t11218;
    (t35908, t35913, t35918, t35951, t35959, t36117)
}
