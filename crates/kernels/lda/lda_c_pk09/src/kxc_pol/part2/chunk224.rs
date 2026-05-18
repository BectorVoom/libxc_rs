//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 224/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk224<F: Float>(t169: F, t633: F, t849: F, t612: F, t616: F, t626: F, t636: F, t653: F, t158: F, t89: F, t155: F, t573: F) -> (F, F, F, F, F, F, F, F) {
    let t851 = t849 * t169 * t633;
    let t856 = F::new(2.0) * t612;
    let t857 = F::new(4.0) / F::new(3.0) * t616;
    let t861 = t856 + t857 + F::new(2.0) * t626 + F::new(2.0) * t636 - F::new(2.0) * t653;
    let t862 = F::new(1.0) / t158;
    let t863 = t861 * t862;
    let t864 = t863 * t89;
    let t870 = F::new(12.992782516386768) * t155 * t573;
    (t851, t856, t857, t861, t862, t863, t864, t870)
}
