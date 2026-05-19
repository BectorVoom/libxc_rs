//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 996/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk996<F: Float>(t1499: F, t2101: F, t9317: F, t3443: F, t802: F, t9330: F, t9332: F, t1988: F, t3203: F, t493: F, t9338: F, t9340: F, t9342: F, t9345: F, t9348: F) -> (F, F, F, F, F, F, F) {
    let t11842 = t1499 * t2101 / F::new(10.0);
    let t11843 = F::new(2.0) / F::new(15.0) * t9317;
    let t11845 = t802 * t3443 / F::new(30.0);
    let t11846 = F::new(4.0) / F::new(135.0) * t9330;
    let t11847 = F::new(2.0) / F::new(45.0) * t9332;
    let t11853 = F::new(2.0) / F::new(15.0) * t493 * t1988 * t3203;
    let t11854 = -t11842 + t11843 - t11845 + t11846 - t11847 + F::cast_from(0.09973633333333333_f64) * t9338 + F::new(0.299209) * t9340 - F::cast_from(0.19947266666666666_f64) * t9342 - t9345 + t9348 + t11853;
    (t11842, t11843, t11845, t11846, t11847, t11853, t11854)
}
