//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 50/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk50(t101: f64, t107: f64, t179: f64, t180: f64, t183: f64, t119: f64) -> (f64, f64) {
    let t187 = 0.619125e-2_f64 * t179 * t180 - 0.79593333333333333331e-1_f64 * t107 * t183 * t101;
    let t188 = t187 * t119;
    (t187, t188)
}
