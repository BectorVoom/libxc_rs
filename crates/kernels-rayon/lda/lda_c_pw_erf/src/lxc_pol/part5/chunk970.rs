//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 970/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk970(t1945: f64, t925: f64, t1953: f64, t817: f64, t1955: f64, t8930: f64, t1284: f64, t4571: f64, t3704: f64, t4505: f64, t12428: f64, t1351: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13714 = t925 * t1945;
    let t13715 = 0.03199259259259259_f64 * t13714;
    let t13731 = t1953 * t817;
    let t13736 = t8930 * t1955;
    let t13749 = t1284 * t4571;
    let t13750 = 8.0_f64 / 45.0_f64 * t13749;
    let t13771 = t4505 * t3704;
    let t13797 = t12428 * t1351;
    (t13714, t13715, t13731, t13736, t13750, t13771, t13797)
}
