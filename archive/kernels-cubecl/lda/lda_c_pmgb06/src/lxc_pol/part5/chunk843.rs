//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 843/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk843<F: Float>(t52: F, t740: F, t933: F, t934: F, t1186: F, t1770: F, t4243: F, t1768: F, t2837: F, t398: F, t419: F, t4238: F) -> (F, F, F, F) {
    let t8047 = F::cast_from(0.6085382050380247_f64) * t933 * t934 * t740 * t52;
    let t8070 = t4243 * t1186 * t1770;
    let t8074 = F::cast_from(0.00010931146159029059_f64) * t1768 * t2837 * t1770;
    let t8077 = t4238 * t398 * t419 * t1770;
    (t8047, t8070, t8074, t8077)
}
