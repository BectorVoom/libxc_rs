//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1280/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1280<F: Float>(t13090: F, t13092: F, t16797: F, t16800: F, t16801: F, t16806: F, t16809: F, t16812: F, t16817: F, t16820: F, t16824: F, t16828: F, t16833: F, t16835: F) -> (F, F, F) {
    let t16836 = F::new(8.0) / F::new(405.0) * t13090;
    let t16837 = F::new(2.0) / F::new(45.0) * t13092;
    let t16838 = t16797 + t16800 + t16801 + t16806 - t16809 - t16812 - t16817 + t16820 + t16824 + t16828 + t16833 - t16835 - t16836 - t16837;
    (t16836, t16837, t16838)
}
