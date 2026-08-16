//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 869/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk869(t696: f64, t8522: f64, t963: f64, t967: f64, t1092: f64, t1108: f64, t4641: f64, t4913: f64, t622: f64, t633: f64, t8697: f64, t8699: f64, t8702: f64, t8704: f64, t8710: f64, t8712: f64, t8714: f64, t8716: f64) -> (f64, f64, f64) {
    let t8798 = 51.94757731704439_f64 * t696 * t963 * t8522 * t967;
    let t8799 = t1108 * t1092;
    let t8814 = 1.0_f64 * t622 * (-2.109916666666667_f64 * t8697 + 20.2552_f64 * t8699 - 7.501925925925926_f64 * t8702 + 6.564185185185186_f64 * t8704 + 3.100395061728395_f64 * t4641 + 0.06825833333333334_f64 * t8710 - 1.0921333333333334_f64 * t8712 + 1.2134814814814814_f64 * t8714 + 1.0617962962962963_f64 * t8716 + 1.3388493827160495_f64 * t4913) * t633;
    (t8798, t8799, t8814)
}
