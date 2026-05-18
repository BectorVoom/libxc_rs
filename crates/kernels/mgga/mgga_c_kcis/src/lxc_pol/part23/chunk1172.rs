//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1172/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1172<F: Float>(t7579: F, t9232: F, t7592: F, t7583: F, t36962: F, t26571: F, t26602: F, t26615: F, t26597: F, t26576: F, t26607: F, t26611: F) -> (F, F, F, F, F, F, F, F) {
    let t92270 = t9232 * t7579;
    let t92271 = t92270 * t7592;
    let t92273 = t92270 * t7583;
    let t92276 = t36962 * t7579 * t7583;
    let t92278 = t26602 * t26571;
    let t92280 = t26602 * t26615;
    let t92282 = t26597 * t26571;
    let t92284 = t26607 * t26576;
    let t92286 = t26607 * t26611;
    (t92271, t92273, t92276, t92278, t92280, t92282, t92284, t92286)
}
