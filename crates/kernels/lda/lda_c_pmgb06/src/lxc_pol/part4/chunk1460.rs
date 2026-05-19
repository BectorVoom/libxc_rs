//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1460/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1460<F: Float>(t14816: F, t35: F, t370: F, t2707: F, t410: F, t360: F, t6973: F, t947: F, t6976: F, t110: F, t6979: F, t2703: F) -> (F, F, F, F, F, F, F, F) {
    let t18741 = t35 * t370 * t14816;
    let t18744 = t410 * t2707;
    let t18745 = t360 * t18744;
    let t18747 = t6973 * t947;
    let t18748 = F::cast_from(1.2991222222222223_f64) * t18747;
    let t18749 = t6976 * t947;
    let t18750 = F::cast_from(0.6495611111111111_f64) * t18749;
    let t18751 = t110 * t6979;
    let t18752 = t360 * t18751;
    let t18754 = t410 * t2703;
    (t18741, t18744, t18745, t18748, t18750, t18751, t18752, t18754)
}
