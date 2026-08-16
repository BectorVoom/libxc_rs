//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1756/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1756(t43881: f64, t56236: f64, t68257: f64, t68399: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64) -> f64 {
    let t90449 = -40.0_f64 / 81.0_f64 * t81230 + 16.0_f64 / 9.0_f64 * t81232 - 16.0_f64 / 27.0_f64 * t68257 - 8.0_f64 / 3.0_f64 * t81234 - 4.0_f64 / 9.0_f64 * t81236 + 40.0_f64 / 9.0_f64 * t89865 - 8.0_f64 * t89869 + 8.0_f64 * t89873 + t89877 / 3.0_f64 - 112.0_f64 / 81.0_f64 * t56236 + t43881 + 16.0_f64 / 9.0_f64 * t68399;
    t90449
}
