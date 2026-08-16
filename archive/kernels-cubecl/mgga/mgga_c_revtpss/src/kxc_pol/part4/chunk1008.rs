//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1008/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1008<F: Float>(t243: F, t9721: F, t268: F, t10688: F, t2479: F, t2652: F, t207: F, t242: F, t240: F, t72: F, t136: F, t2476: F) -> (F, F, F, F) {
    let t10689 = t9721 * t243;
    let t10690 = t10689 * t268;
    let t10692 = F::cast_from(0.20082057720118594944e-6_f64) * t10688 * t10690;
    let t10693 = t2652 * t2479;
    let t10696 = F::cast_from(1.0_f64) / t242 / t207;
    let t10697 = t240 * t10696;
    let t10698 = t10697 * t72;
    let t10703 = t2476 * t136;
    (t10692, t10693, t10698, t10703)
}
