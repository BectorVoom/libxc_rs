//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1351/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1351(t1082: f64, t15648: f64, t3291: f64, t4757: f64, t3059: f64, t5004: f64, t16426: f64, t3318: f64, t1043: f64, t1089: f64, t4930: f64, t15717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16479 = t1082 * t15648;
    let t16482 = t3291 * t4757;
    let t16485 = t5004 * t3059;
    let t16488 = t16426 * t3318;
    let t16496 = t4930 * t1043 * t1089;
    let t16499 = t1082 * t15717;
    (t16479, t16482, t16485, t16488, t16496, t16499)
}
