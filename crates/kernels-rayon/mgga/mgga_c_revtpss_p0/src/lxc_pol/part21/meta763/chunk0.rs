//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2709/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2709(t39438: f64, t1469: f64, t2608: f64, t4401: f64, t606: f64, t10428: f64, t4308: f64, t14425: f64, t705: f64, t707: f64, t10356: f64, t1522: f64, t157: f64) -> (f64, f64, f64, f64, f64) {
    let t49873 = 0.48796115851357829289e-1_f64 * t39438;
    let t49876 = t4401 * t2608 * t1469 * t606;
    let t49877 = 36.0_f64 * t49876;
    let t49879 = 12.0_f64 * t10428 * t4308;
    let t49880 = t705 * t14425;
    let t49882 = 12.0_f64 * t49880 * t707;
    let t49885 = 24.0_f64 * t10356 * t157 * t1522;
    (t49873, t49877, t49879, t49882, t49885)
}
