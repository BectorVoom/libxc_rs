//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 519/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk519(t2221: f64, t35: f64, t365: f64, t780: f64, t350: f64, t1282: f64, t769: f64) -> (f64, f64, f64, f64) {
    let t2222 = t35 * t2221;
    let t2226 = t365 * t780;
    let t2227 = t2226 * t350;
    let t2229 = t1282 * t769;
    (t2222, t2226, t2227, t2229)
}
