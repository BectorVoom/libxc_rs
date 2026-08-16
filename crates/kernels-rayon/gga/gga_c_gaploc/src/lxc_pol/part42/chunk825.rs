//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 825/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk825(t13287: f64, t64: f64, t39624: f64, t39626: f64, t39632: f64, t39646: f64, t39648: f64, t39650: f64, t2268: f64, t35901: f64, t894: f64, t426: f64, t44386: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44592 = 4.0_f64 / 3.0_f64 * t13287 * t64;
    let t44595 = 7.0_f64 / 256.0_f64 * t39624;
    let t44596 = 63.0_f64 / 8192.0_f64 * t39626;
    let t44597 = 63.0_f64 / 524288.0_f64 * t39632;
    let t44598 = 21.0_f64 / 524288.0_f64 * t39646;
    let t44599 = 21.0_f64 / 8192.0_f64 * t39648;
    let t44600 = 7.0_f64 / 768.0_f64 * t39650;
    let t44618 = 0.56910013271352299198e-1_f64 * t2268 * t894 * t35901;
    let t44622 = 0.28455006635676149599e-1_f64 * t2268 * t535 * t44386 * t426;
    (t44592, t44595, t44596, t44597, t44598, t44599, t44600, t44618, t44622)
}
