//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 697/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk697(t27103: f64, t446: f64, t27064: f64, t9073: f64, t1369: f64, t376: f64, t6665: f64, t27066: f64, t27070: f64, t27075: f64, t27079: f64, t27084: f64, t27089: f64, t27094: f64, t27098: f64, t27101: f64) -> (f64, f64, f64, f64, f64) {
    let t27104 = t446 * t27103;
    let t27106 = t9073 * t27064;
    let t27107 = t446 * t27106;
    let t27110 = t1369 * t376 * t6665;
    let t27112 = -t27066 / 3.0_f64 - t27070 / 3.0_f64 + t27075 / 9.0_f64 - t27079 / 12.0_f64 - t27084 / 12.0_f64 + t27089 / 4.0_f64 + t27094 / 4.0_f64 - 2.0_f64 / 3.0_f64 * t27098 - 2.0_f64 / 3.0_f64 * t27101 + 2.0_f64 / 9.0_f64 * t27104 - 2.0_f64 / 3.0_f64 * t27107 - t27110 / 3.0_f64;
    (t27104, t27106, t27107, t27110, t27112)
}
