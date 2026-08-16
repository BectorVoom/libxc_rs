//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1128/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1128(t2016: f64, t9630: f64, t1327: f64, t507: f64, t8888: f64, t142: f64, t6289: f64, t7436: f64, t6300: f64, t6309: f64, t6313: f64, t2020: f64, t9761: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39508 = t2016 * t9630;
    let t39511 = t8888 * t507 * t1327;
    let t39514 = t7436 * t142 * t6289;
    let t39517 = t7436 * t142 * t6300;
    let t39520 = t7436 * t142 * t6309;
    let t39525 = t8888 * t142 * t6313;
    let t39527 = t2020 * t9761;
    (t39508, t39511, t39514, t39517, t39520, t39525, t39527)
}
