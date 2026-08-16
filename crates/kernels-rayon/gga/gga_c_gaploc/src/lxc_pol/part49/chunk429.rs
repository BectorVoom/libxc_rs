//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 429/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk429(t3695: f64, t492: f64, t105: f64, t3124: f64, t3132: f64, t3329: f64, t3346: f64, t3349: f64, t3353: f64, t3357: f64, t3692: f64, t189: f64, t3689: f64) -> (f64, f64, f64) {
    let t3696 = t492 * t3695;
    let t3699 = t3329 + 0.28455006635676149599e-1_f64 * t105 * t3692 + t3124 - t3349 + t3346 - t3353 - t3132 + t3357 - 0.28455006635676149599e-1_f64 * t105 * t3696;
    let t3701 = t189 * t3689;
    (t3696, t3699, t3701)
}
