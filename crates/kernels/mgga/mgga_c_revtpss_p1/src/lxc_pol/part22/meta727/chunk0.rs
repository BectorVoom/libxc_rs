//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2783/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2783<F: Float>(t39875: F, t745: F, t9417: F, t760: F, t2596: F, t39871: F, t10587: F, t2626: F, t2523: F, t9425: F, t2389: F, t37: F) -> (F, F, F, F, F, F, F) {
    let t40192 = t9417 * t39875 * t745;
    let t40194 = F::cast_from(0.14035736694323150897e2_f64) * t760 * t40192;
    let t40196 = t2596 * t39871 * t745;
    let t40198 = F::cast_from(0.35089341735807877242e1_f64) * t760 * t40196;
    let t40203 = t10587 * t2626;
    let t40205 = t2523 * t9425;
    let t40207 = t37 * t2389;
    (t40192, t40194, t40196, t40198, t40203, t40205, t40207)
}
