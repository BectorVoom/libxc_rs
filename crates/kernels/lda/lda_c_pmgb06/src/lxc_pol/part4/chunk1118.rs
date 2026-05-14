//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1118/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1118<F: Float>(t12519: F, t16527: F, t5083: F, t4790: F, t831: F, t12043: F, t1981: F, t496: F, t851: F, t2489: F, t3223: F, t5432: F, t844: F, t16724: F, t16727: F, t16729: F, t16731: F, t16734: F, t16735: F, t16736: F, t16737: F, t16738: F, t16739: F) -> (F, F, F, F, F, F) {
    let t16742 = 8.0 / 27.0 * t5083 * t12519 * t16527;
    let t16743 = t831 * t4790;
    let t16744 = 4.0 / 45.0 * t16743;
    let t16748 = 4.0 / 45.0 * t1981 * t496 * t12043 * t851;
    let t16749 = t3223 * t2489;
    let t16750 = 4.0 / 405.0 * t16749;
    let t16752 = t5432 * t844 / 15.0;
    let t16753 = t16724 + t16727 + t16729 + t16731 + t16734 + t16735 + t16736 + t16737 + t16738 + t16739 + t16742 + t16744 + t16748 + t16750 + t16752;
    (t16742, t16744, t16748, t16750, t16752, t16753)
}
