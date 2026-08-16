//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1062/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1062<F: Float>(t4588: F, t517: F, t2992: F, t493: F, t1925: F, t3223: F, t1423: F, t5238: F, t1908: F, t3220: F, t1972: F, t2984: F) -> (F, F, F, F, F) {
    let t12617 = t4588 * t517;
    let t12620 = t493 * t12617 * t2992 / F::cast_from(9.0_f64);
    let t12621 = t3223 * t1925;
    let t12622 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t12621;
    let t12623 = t1423 * t5238;
    let t12624 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12623;
    let t12625 = t3220 * t1908;
    let t12626 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12625;
    let t12628 = t1972 * t2984 / F::cast_from(15.0_f64);
    (t12620, t12622, t12624, t12626, t12628)
}
