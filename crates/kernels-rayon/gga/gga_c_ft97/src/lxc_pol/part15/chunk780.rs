//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 780/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk780(t21416: f64, t9707: f64, t27: f64, t89: f64, t3717: f64, t5053: f64, t193: f64, t18145: f64, t18148: f64, t18168: f64, t18171: f64, t18174: f64, t21402: f64, t21406: f64, t21410: f64, t21414: f64) -> (f64, f64, f64, f64, f64) {
    let t21417 = t9707 * t21416;
    let t21419 = t89 * t27 * t21417;
    let t21420 = t3717 * t5053;
    let t21422 = t89 * t193 * t21420;
    let t21428 = -t21402 / 6.0_f64 - t21406 / 3.0_f64 - t21410 / 3.0_f64 - t21414 / 18.0_f64 - t21419 + t21422 + t18148 / 6.0_f64 - t18145 / 3.0_f64 + t18168 / 18.0_f64 - t18171 / 9.0_f64 + t18174 / 27.0_f64;
    (t21417, t21419, t21420, t21422, t21428)
}
