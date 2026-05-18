//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 691/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk691<F: Float>(t2775: F, t776: F, t101: F, t440: F, t770: F, t2765: F, t4465: F, t2455: F, t668: F, t2325: F, t406: F, t2329: F, t92: F) -> (F, F, F, F, F, F, F, F) {
    let t6153 = t776 * t2775;
    let t6154 = t101 * t6153;
    let t6155 = t770 * t440;
    let t6156 = t2765 * t6155;
    let t6161 = F::new(8.0) / F::new(135.0) * t4465;
    let t6162 = t2455 * t668;
    let t6164 = t406 * t2325;
    let t6169 = t92 * t2329;
    (t6153, t6154, t6155, t6156, t6161, t6162, t6164, t6169)
}
