//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 807/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk807(t9484: f64, t9543: f64, t520: f64, t512: f64, t1333: f64, t3857: f64, t2626: f64, t676: f64, t3869: f64, t2434: f64, t762: f64, t186: f64, t685: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9544 = t9484 + t9543;
    let t9545 = t520 * t9544;
    let t9546 = t512 * t9545;
    let t9569 = 60.0_f64 * t3857 * t1333;
    let t9572 = t676 * t2626;
    let t9574 = 0.32530743900905219526e-1_f64 * t3869 * t9572;
    let t9575 = t2434 * t762;
    let t9577 = 0.21687162600603479684e-1_f64 * t3869 * t9575;
    let t9586 = t685 * t793 * t186;
    (t9544, t9546, t9569, t9572, t9574, t9575, t9577, t9586)
}
