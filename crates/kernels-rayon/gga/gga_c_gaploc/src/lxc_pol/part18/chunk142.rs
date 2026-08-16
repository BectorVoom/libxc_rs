//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 142/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk142(t397: f64, t513: f64, t10: f64, t101: f64, t107: f64, t179: f64, t180: f64, t183: f64, t415: f64, t503: f64, t507: f64, t510: f64) -> (f64, f64) {
    let t514 = t513 * t397;
    let t523 = 0.619125e-2_f64 * t503 * t180 - 0.123825e-1_f64 * t507 * t510 - 0.619125e-2_f64 * t179 * t514 - 0.53062222222222222221e-1_f64 * t107 * t10 * t101 - 0.79593333333333333331e-1_f64 * t107 * t183 * t415;
    (t514, t523)
}
