//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2368/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2368<F: Float>(t40196: F, t760: F, t10587: F, t2626: F, t2523: F, t9425: F, t2389: F, t37: F, t2394: F, t2475: F, t10069: F, t10929: F) -> (F, F, F, F, F, F) {
    let t40198 = F::cast_from(0.35089341735807877242e1_f64) * t760 * t40196;
    let t40203 = t10587 * t2626;
    let t40205 = t2523 * t9425;
    let t40207 = t37 * t2389;
    let t40236 = t2475 * t2394;
    let t40267 = t10069 * t10929;
    (t40198, t40203, t40205, t40207, t40236, t40267)
}
