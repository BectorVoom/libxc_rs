//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 712/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk712(t5271: f64, t9708: f64, t5259: f64, t9704: f64, t10053: f64, t3814: f64, t558: f64, t8975: f64, t1743: f64, t665: f64, t645: f64, t9908: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10135 = t5271 * t9708;
    let t10137 = t5259 * t9704;
    let t10141 = t3814 * t10053;
    let t10143 = t8975 * t558;
    let t10148 = t665 * t1743;
    let t10151 = t9908 * t645;
    (t10135, t10137, t10141, t10143, t10148, t10151)
}
