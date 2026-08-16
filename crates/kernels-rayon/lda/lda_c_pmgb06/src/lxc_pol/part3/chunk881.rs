//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 881/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk881(t248: f64, t258: f64, t8887: f64, t8925: f64, t8990: f64, t9033: f64, t1200: f64, t718: f64, t2803: f64, t81: f64, t199: f64, t2813: f64, t566: f64) -> (f64, f64, f64, f64, f64) {
    let t9037 = t248 * t258 * (t8887 + t8925 + t8990 + t9033);
    let t9045 = t718 * t1200;
    let t9047 = t81 * t2803;
    let t9048 = t9047 * t199;
    let t9050 = t2813 * t566;
    (t9037, t9045, t9047, t9048, t9050)
}
