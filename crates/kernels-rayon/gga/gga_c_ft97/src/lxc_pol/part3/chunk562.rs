//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 562/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk562(t4431: f64, t464: f64, t463: f64, t1800: f64, t24: f64, t4436: f64, t4495: f64, t469: f64, t1773: f64, t3125: f64, t3144: f64, t4512: f64, t4515: f64, t4519: f64, t462: f64, t92: f64) -> (f64, f64, f64, f64, f64) {
    let t4522 = t464 * t4431;
    let t4523 = t463 * t4522;
    let t4527 = t24 * t1800 * t4436;
    let t4531 = t24 * t469 * t4495;
    let t4533 = t1773 + 2.0_f64 / 9.0_f64 * t3125 + 2.0_f64 / 3.0_f64 * t3144 - 2.0_f64 / 9.0_f64 * t462 * t4512 + 2.0_f64 / 3.0_f64 * t462 * t4515 + 2.0_f64 / 3.0_f64 * t462 * t4519 - t462 * t4523 / 3.0_f64 + 2.0_f64 * t92 * t4527 - t92 * t4531;
    (t4522, t4523, t4527, t4531, t4533)
}
