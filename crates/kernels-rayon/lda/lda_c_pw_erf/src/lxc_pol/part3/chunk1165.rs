//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1165/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1165(t331: f64, t4991: f64, t174: f64, t4697: f64, t9810: f64, t1950: f64, t925: f64, t325: f64, t4685: f64, t1945: f64, t11: f64, t12282: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13705 = t331 * t4991;
    let t13708 = t174 * t9810 * t4697;
    let t13710 = t925 * t1950;
    let t13712 = t325 * t4685;
    let t13714 = t925 * t1945;
    let t13715 = 0.03199259259259259_f64 * t13714;
    let t13717 = t11 * t557 * t12282;
    (t13705, t13708, t13710, t13712, t13714, t13715, t13717)
}
