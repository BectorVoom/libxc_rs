//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 862/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk862<F: Float>(t3947: F, t654: F, t1090: F, t1101: F, t1023: F, t1035: F, t350: F, t634: F, t1040: F, t1043: F, t632: F, t138: F, t3875: F, t3885: F) -> (F, F, F, F, F) {
    let t8614 = t3947 * t654;
    let t8616 = t1101 * t1090;
    let t8621 = F::new(0.4274) * t350 * t1023 * t1035 * t634;
    let t8626 = F::cast_from(3.436719018870595_f64) * t350 * t1040 * t1035 * t1043 * t632;
    let t8629 = F::new(0.4274) * t138 * t3885 * t3875;
    (t8614, t8616, t8621, t8626, t8629)
}
