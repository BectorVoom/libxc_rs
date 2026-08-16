//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 878/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk878(t334: f64, t8218: f64, t913: f64, t904: f64, t907: f64, t319: f64, t4606: f64, t5021: f64, t8141: f64, t8143: f64, t8146: f64, t8149: f64, t8155: f64, t8157: f64, t8159: f64, t8161: f64) -> (f64, f64, f64) {
    let t8221 = 6.0_f64 * t913 * t8218 * t334;
    let t8224 = 48.24547296645331_f64 * t904 * t8218 * t907;
    let t8238 = 1.0_f64 * t319 * (-2.109916666666667_f64 * t8141 + 20.2552_f64 * t8143 - 7.501925925925926_f64 * t8146 + 6.564185185185186_f64 * t8149 + 3.100395061728395_f64 * t4606 + 0.06825833333333334_f64 * t8155 - 1.0921333333333334_f64 * t8157 + 1.2134814814814814_f64 * t8159 + 1.0617962962962963_f64 * t8161 + 1.3388493827160495_f64 * t5021) * t334;
    (t8221, t8224, t8238)
}
