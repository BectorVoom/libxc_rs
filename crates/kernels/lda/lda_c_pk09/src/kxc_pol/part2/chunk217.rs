//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 217/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk217<F: Float>(t721: F, t810: F, t666: F, t670: F, t612: F, t616: F, t626: F, t636: F, t653: F, t676: F, t681: F, t687: F) -> (F, F, F, F, F, F) {
    let t812 = F::cast_from(19.489173774580152_f64) * t810 * t721;
    let t813 = F::new(12.0) * t666;
    let t814 = F::new(8.0) * t670;
    let t818 = F::cast_from(0.821419393556371_f64) * t612;
    let t819 = F::cast_from(0.5476129290375806_f64) * t616;
    let t823 = t813 + t814 + F::new(12.0) * t676 + F::new(12.0) * t681 - F::new(12.0) * t687 + t818 + t819 + F::cast_from(0.821419393556371_f64) * t626 + F::cast_from(0.821419393556371_f64) * t636 - F::cast_from(0.821419393556371_f64) * t653;
    (t812, t813, t814, t818, t819, t823)
}
