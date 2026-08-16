//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3476/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3476<F: Float>(t15547: F, t4725: F, t1642: F, t52921: F, t4729: F, t4734: F, t64465: F, t64467: F, t64471: F, t64475: F, t64483: F, t65402: F, t65404: F, t65408: F, t65413: F) -> (F, F, F, F, F) {
    let t65415 = F::cast_from(0.46785788981077169656e1_f64) * t15547 * t4725;
    let t65417 = F::cast_from(0.11696447245269292414e1_f64) * t52921 * t1642;
    let t65419 = F::cast_from(0.23392894490538584828e1_f64) * t15547 * t4729;
    let t65421 = F::cast_from(0.69263436422725855034e2_f64) * t15547 * t4734;
    let t65422 = t64465 + t65402 + t64467 - t65404 + t65408 - t65413 + t65415 - t65417 - t65419 - t65421 + t64471 + t64475 + t64483;
    (t65415, t65417, t65419, t65421, t65422)
}
