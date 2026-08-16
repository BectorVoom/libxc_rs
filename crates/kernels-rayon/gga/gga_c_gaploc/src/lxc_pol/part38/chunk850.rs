//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 850/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk850(t39624: f64, t39626: f64, t39632: f64, t39646: f64, t39648: f64, t39650: f64, t42114: f64, t42115: f64, t44590: f64, t44592: f64, t44593: f64, t493: f64) -> (f64, f64) {
    let t44595 = 7.0_f64 / 256.0_f64 * t39624;
    let t44596 = 63.0_f64 / 8192.0_f64 * t39626;
    let t44597 = 63.0_f64 / 524288.0_f64 * t39632;
    let t44598 = 21.0_f64 / 524288.0_f64 * t39646;
    let t44599 = 21.0_f64 / 8192.0_f64 * t39648;
    let t44600 = 7.0_f64 / 768.0_f64 * t39650;
    let t44601 = t44590 - t44592 + t44593 / 2.0_f64 + t42114 - t42115 + t44595 + t44596 - t44597 + t44598 - t44599 - t44600;
    let t44609 = t493 * t44601;
    (t44601, t44609)
}
