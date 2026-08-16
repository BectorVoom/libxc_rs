//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1080/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1080(t11516: f64, t11520: f64, t11522: f64, t11525: f64, t11526: f64, t2633: f64, t2644: f64, t2828: f64, t2835: f64, t4028: f64, t4030: f64, t4032: f64, t4034: f64) -> f64 {
    let t19353 = 0.39503346997227602814e-1_f64 * t4028 + t11516 - 0.2077903092681775651e3_f64 * t2633 + 0.14649157844805236043e-2_f64 * t4030 - t4032 + 12.0_f64 * t4034 - t11520 + 6.0_f64 * t2644 + t11522 + 2.0_f64 * t2828 + t11525 - t11526 + 0.70178683471615754484e1_f64 * t2835;
    t19353
}
