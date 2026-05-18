//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1062/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1062<F: Float>(t16181: F, t6600: F, t802: F, t6572: F, t1887: F, t2631: F, t161: F, t166: F, t2623: F, t4801: F, t1908: F, t6127: F) -> (F, F, F, F, F, F) {
    let t19712 = t16181 / F::new(15.0);
    let t19714 = t802 * t6600 / F::new(5.0);
    let t19716 = t802 * t6572 / F::new(5.0);
    let t19718 = t1887 * t2631 / F::new(5.0);
    let t19722 = t161 * t166 * t4801 * t2623 / F::new(10.0);
    let t19724 = t6127 * t1908 / F::new(15.0);
    (t19712, t19714, t19716, t19718, t19722, t19724)
}
