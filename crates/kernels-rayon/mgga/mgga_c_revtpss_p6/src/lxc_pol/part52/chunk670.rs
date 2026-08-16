//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 670/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk670(t1940: f64, t1963: f64, t2403: f64, t30: f64, t605: f64, t7010: f64, t7087: f64, t7091: f64, t7092: f64, t1976: f64, t994: f64, t343: f64, t613: f64) -> (f64, f64, f64) {
    let t7099 = 3.0_f64 / 2.0_f64 * t2403 * t1963 * t7010 + t1940 * t7087 * t30 / 2.0_f64 - t1940 * t7091 * t7092 / 2.0_f64 + t1940 * t1963 * t605 / 2.0_f64;
    let t7102 = t994 * t1976;
    let t7105 = t613 * t343;
    (t7099, t7102, t7105)
}
