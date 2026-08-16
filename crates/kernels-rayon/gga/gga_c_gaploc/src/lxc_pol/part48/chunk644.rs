//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 644/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk644(t10661: f64, t11568: f64, t3611: f64, t471: f64, t64: f64, t9664: f64, t9666: f64, t9674: f64, t9676: f64) -> f64 {
    let t11576 = t11568 * t471 - 4.0_f64 / 3.0_f64 * t3611 * t64 + t10661 - 7.0_f64 / 128.0_f64 * t9664 + 21.0_f64 / 4096.0_f64 * t9666 - 7.0_f64 / 4096.0_f64 * t9674 + 7.0_f64 / 384.0_f64 * t9676;
    t11576
}
