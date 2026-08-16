//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2158/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2158(t22685: f64, t22881: f64, t6330: f64, t6637: f64, t22893: f64, t28142: f64, t80681: f64, t2006: f64, t6387: f64, t28143: f64, t80727: f64, t6414: f64) -> (f64, f64, f64, f64, f64) {
    let t97158 = t22685 * t6637 * t22881 * t6330;
    let t97161 = t80681 * t22893 * t28142;
    let t97172 = t2006 * t6387;
    let t97179 = t80727 * t28143;
    let t97181 = t2006 * t6414;
    (t97158, t97161, t97172, t97179, t97181)
}
