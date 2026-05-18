//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1058/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1058<F: Float>(t16051: F, t16053: F, t16055: F, t16057: F, t16089: F, t1969: F, t6127: F, t6584: F, t802: F, t1887: F, t2650: F, t132: F, t137: F, t1395: F, t7801: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19679 = F::new(4.0) / F::new(45.0) * t16051;
    let t19680 = F::new(4.0) / F::new(15.0) * t16053;
    let t19681 = F::new(4.0) / F::new(9.0) * t16055;
    let t19682 = F::new(16.0) / F::new(45.0) * t16057;
    let t19683 = F::new(2.0) / F::new(15.0) * t16089;
    let t19685 = t6127 * t1969 / F::new(5.0);
    let t19687 = t802 * t6584 / F::new(10.0);
    let t19689 = t1887 * t2650 / F::new(10.0);
    let t19693 = t132 * t137 * t1395 * t7801 / F::new(30.0);
    (t19679, t19680, t19681, t19682, t19683, t19685, t19687, t19689, t19693)
}
