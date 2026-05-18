//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1272/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1272<F: Float>(t12913: F, t12915: F, t12917: F, t12919: F, t12519: F, t16527: F, t5083: F, t4790: F, t831: F, t12043: F, t1981: F, t496: F, t851: F) -> (F, F, F, F, F, F, F) {
    let t16736 = F::new(8.0) / F::new(135.0) * t12913;
    let t16737 = F::new(8.0) / F::new(135.0) * t12915;
    let t16738 = F::new(4.0) / F::new(135.0) * t12917;
    let t16739 = F::new(4.0) / F::new(81.0) * t12919;
    let t16742 = F::new(8.0) / F::new(27.0) * t5083 * t12519 * t16527;
    let t16743 = t831 * t4790;
    let t16744 = F::new(4.0) / F::new(45.0) * t16743;
    let t16748 = F::new(4.0) / F::new(45.0) * t1981 * t496 * t12043 * t851;
    (t16736, t16737, t16738, t16739, t16742, t16744, t16748)
}
