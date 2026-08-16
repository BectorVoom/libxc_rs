//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 939/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk939(t1960: f64, t5379: f64, t7980: f64, t8397: f64, t2132: f64, t2138: f64, t322: f64, t8993: f64, t2147: f64, t2341: f64, t7885: f64, t864: f64) -> (f64, f64, f64, f64) {
    let t33496 = 0.13170898365871023197e1_f64 * t1960 * t5379;
    let t33500 = 0.17347256376410398924e1_f64 * t8397 * t7980;
    let t33504 = 0.17347256376410398924e1_f64 * t2138 * t2132 * t8993 * t322;
    let t33507 = t7885 * t2147 * t2341 * t864;
    (t33496, t33500, t33504, t33507)
}
