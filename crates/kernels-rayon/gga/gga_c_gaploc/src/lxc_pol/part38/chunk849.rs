//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 849/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk849(t39624: f64, t39626: f64, t39632: f64, t39637: f64, t39642: f64, t39646: f64, t39648: f64, t39650: f64, t471: f64, t13287: f64, t64: f64, t11210: f64, t871: f64) -> (f64, f64, f64) {
    let t44590 = (21.0_f64 / 256.0_f64 * t39624 + 357.0_f64 / 8192.0_f64 * t39626 - 189.0_f64 / 131072.0_f64 * t39632 + 189.0_f64 / 8388608.0_f64 * t39637 - 63.0_f64 / 8388608.0_f64 * t39642 + 63.0_f64 / 131072.0_f64 * t39646 - 119.0_f64 / 8192.0_f64 * t39648 - 7.0_f64 / 256.0_f64 * t39650) * t471;
    let t44592 = 4.0_f64 / 3.0_f64 * t13287 * t64;
    let t44593 = t11210 * t871;
    (t44590, t44592, t44593)
}
