//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 261/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk261<F: Float>(t519: F, t799: F, t538: F, t789: F, t25: F, t531: F, t536: F, t791: F) -> (F, F, F) {
    let t801 = F::new(4.0) / F::new(45.0) * t519 * t799;
    let t803 = t538 * t789;
    let t806 = -t531 - F::new(0.035991666666666665) * t791 - t536 - F::new(0.006666666666666667) * t25 * t803;
    (t801, t803, t806)
}
