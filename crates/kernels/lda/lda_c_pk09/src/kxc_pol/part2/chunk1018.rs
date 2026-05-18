//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1018/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1018<F: Float>(t11045: F, t6442: F, t11023: F, t9700: F, t2723: F, t9704: F, t11004: F, t11039: F, t11042: F, t1193: F, t1197: F, t1713: F, t2711: F, t2724: F, t620: F, t6409: F) -> F {
    let t11046 = t11045 * t6442;
    let t11049 = t11023 * t9700;
    let t11052 = t2723 * t9704;
    let t11055 = t11004 * t1193 + t2711 * t620 * t1197 + t11039 - t11042 + F::new(1.28) * t6409 * t2724 - F::new(1.28) * t1713 * t11046 + F::new(2.56) * t1713 * t11049 - F::new(1.28) * t1713 * t11052;
    t11055
}
