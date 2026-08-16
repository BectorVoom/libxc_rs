//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 475/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk475(t3474: f64, t3587: f64, t160: f64, t3539: f64, t1023: f64, t1058: f64, t149: f64, t165: f64, t3313: f64, t3414: f64, t3484: f64, t3566: f64, t3579: f64, t3583: f64, t564: f64, t614: f64) -> (f64, f64, f64) {
    let t3588 = t3474 + t3587;
    let t3590 = t3539 * t160;
    let t3596 = -t1023 * t614 - t1058 * t564 - t149 * t3588 - t165 * t3313 - t165 * t3414 + 4.0_f64 * t3484 - 2.0_f64 * t3566 - 2.0_f64 * t3579 - 2.0_f64 * t3583 + 2.0_f64 * t3590;
    (t3588, t3590, t3596)
}
