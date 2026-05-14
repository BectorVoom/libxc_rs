//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1249/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1249<F: Float>(t18744: F, t360: F, t6973: F, t947: F, t6976: F, t110: F, t6979: F, t2703: F, t410: F, t6967: F, t6970: F, t18725: F, t18729: F, t18732: F, t18735: F, t18737: F, t18741: F) -> (F, F, F, F, F) {
    let t18745 = t360 * t18744;
    let t18747 = t6973 * t947;
    let t18748 = 1.2991222222222223 * t18747;
    let t18749 = t6976 * t947;
    let t18750 = 0.6495611111111111 * t18749;
    let t18751 = t110 * t6979;
    let t18752 = t360 * t18751;
    let t18754 = t410 * t2703;
    let t18755 = t360 * t18754;
    let t18757 = t6967 * t947;
    let t18759 = t6970 * t947;
    let t18761 = -0.48968 * t18725 - t18729 + t18732 + t18735 - t360 * t18737 / 2.0 + 3.0 * t360 * t18741 - 2.0 / 9.0 * t18745 + t18748 - t18750 + t18752 / 3.0 + 2.0 / 3.0 * t18755 + 3.91744 * t18757 - 0.97936 * t18759;
    (t18748, t18750, t18751, t18754, t18761)
}
