//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 516/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk516(t123: f64, t9060: f64, t488: f64, t158: f64, t3116: f64) -> (f64, f64, f64) {
    let t9061 = t9060 * t123;
    let t9062 = t9061 * t488;
    let t9065 = t158 * t3116;
    (t9061, t9062, t9065)
}
