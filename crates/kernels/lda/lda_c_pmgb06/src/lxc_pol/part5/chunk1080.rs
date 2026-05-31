//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1080/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1080<F: Float>(t12456: F, t12465: F, t16380: F, t16383: F, t477: F, t7458: F, t5077: F, t5094: F, t12991: F, t19618: F, t12995: F, t5083: F) -> (F, F, F, F, F, F, F, F) {
    let t19985 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t12456;
    let t19986 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t12465;
    let t19987 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16380;
    let t19988 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16383;
    let t19989 = t7458 * t477;
    let t19992 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5077 * t5094 * t19989;
    let t19995 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t5077 * t12991 * t19618;
    let t19998 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5083 * t12995 * t19618;
    (t19985, t19986, t19987, t19988, t19989, t19992, t19995, t19998)
}
