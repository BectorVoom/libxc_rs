//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 373/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk373(t1098: f64, t1138: f64, t1597: f64, t163: f64, t169: f64, t234: f64, t717: f64, t299: f64, t616: f64, t230: f64, t598: f64, t226: f64, t610: f64) -> (f64, f64, f64, f64, f64) {
    let t1599 = 0.0004954275694490498_f64 * t1098 * t1138 * t1597;
    let t1603 = 0.02394846802050922_f64 * t169 * t717 * t234 * t163;
    let t1606 = t169 * t299 * t616 * t163;
    let t1608 = t598 * t230;
    let t1611 = 8.0_f64 / 3.0_f64 * t226 * t610;
    (t1599, t1603, t1606, t1608, t1611)
}
