//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1154/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1154(t3832: f64, t571: f64, t593: f64, t7414: f64, t1472: f64, t7716: f64, t16305: f64, t743: f64, t2017: f64, t34: f64, t6365: f64, t4868: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21204 = 8.0_f64 / 9.0_f64 * t571 * t3832 * t7414 * t593;
    let t21206 = 4.0_f64 / 9.0_f64 * t1472 * t7716;
    let t21207 = t16305 * t743;
    let t21210 = 4.0_f64 / 9.0_f64 * t571 * t2017 * t21207;
    let t21211 = t6365 * t34;
    let t21214 = 8.0_f64 / 9.0_f64 * t571 * t4868 * t21211;
    (t21204, t21206, t21207, t21210, t21211, t21214)
}
