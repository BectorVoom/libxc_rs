//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 999/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk999<F: Float>(t1092: F, t1108: F, t4641: F, t4913: F, t622: F, t633: F, t8697: F, t8699: F, t8702: F, t8704: F, t8710: F, t8712: F, t8714: F, t8716: F) -> (F, F) {
    let t8799 = t1108 * t1092;
    let t8814 = F::new(1.0) * t622 * (-F::cast_from(2.109916666666667_f64) * t8697 + F::new(20.2552) * t8699 - F::cast_from(7.501925925925926_f64) * t8702 + F::cast_from(6.564185185185186_f64) * t8704 + F::cast_from(3.100395061728395_f64) * t4641 + F::cast_from(0.06825833333333334_f64) * t8710 - F::cast_from(1.0921333333333334_f64) * t8712 + F::cast_from(1.2134814814814814_f64) * t8714 + F::cast_from(1.0617962962962963_f64) * t8716 + F::cast_from(1.3388493827160495_f64) * t4913) * t633;
    (t8799, t8814)
}
