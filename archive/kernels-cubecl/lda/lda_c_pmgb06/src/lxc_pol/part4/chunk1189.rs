//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1189/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1189<F: Float>(t132: F, t137: F, t15544: F, t15563: F, t15585: F, t15611: F, t15626: F, t15641: F, t15662: F, t15692: F, t465: F, t15496: F, t15498: F, t15501: F, t15506: F, t15509: F, t15510: F, t15511: F, t15516: F, t15518: F, t15520: F, t15522: F, t15524: F, t15526: F, t15527: F) -> (F, F) {
    let t15699 = t132 * t137 * t465 * (t15544 + t15563 + t15585 + t15611 + t15626 + t15641 + t15662 + t15692) / F::cast_from(30.0_f64);
    let t15700 = -t15496 - t15498 - t15501 - t15506 - t15509 + t15510 + t15511 - t15516 + t15518 - t15520 - t15522 + t15524 - t15526 + t15527 - t15699;
    (t15699, t15700)
}
