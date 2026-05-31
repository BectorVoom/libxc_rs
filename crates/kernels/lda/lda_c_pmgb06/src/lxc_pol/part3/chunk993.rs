//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 993/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk993<F: Float>(t2002: F, t3191: F, t9291: F, t9293: F, t9295: F, t9297: F, t1179: F, t4068: F, t871: F, t11790: F, t11793: F, t11795: F, t11796: F, t11799: F, t11802: F) -> (F, F, F, F, F, F) {
    let t11804 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2002 * t3191;
    let t11805 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t9291;
    let t11806 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t9293;
    let t11807 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9295;
    let t11808 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9297;
    let t11810 = t871 * t1179 * t4068;
    let t11812 = -t11790 - t11793 - t11795 + F::cast_from(0.09973633333333333_f64) * t11796 + t11799 + t11802 - t11804 - t11805 - t11806 + t11807 - t11808 + F::cast_from(0.001515438175925926_f64) * t11810;
    (t11804, t11805, t11806, t11807, t11808, t11812)
}
