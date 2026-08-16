//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 458/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk458(t50: f64, t34: f64, t52: f64, t1789: f64, t352: f64, t462: f64, t1788: f64, t59: f64, zeta_threshold: f64) -> (f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t1792 = t52 * t34;
    let t1796 = piecewise3(t51, 0.0_f64, 4.0_f64 / 9.0_f64 * t1789 * t352 - 8.0_f64 / 3.0_f64 * t1792 * t462);
    let t1798 = (t1788 + t1796) * t59;
    (t1792, t1798)
}
