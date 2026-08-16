//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1222/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1222(t1583: f64, t30: f64, t1468: f64, t1940: f64, t1963: f64, t2403: f64, t7091: f64, t7750: f64, t7783: f64, t1544: f64, t207: f64, t7782: f64) -> (f64, f64, f64, f64) {
    let t7787 = t30 * t1583;
    let t7794 = 3.0_f64 / 2.0_f64 * t2403 * t7750 + t1940 * t7783 * t30 / 2.0_f64 - t1940 * t7091 * t7787 / 2.0_f64 + t1940 * t1963 * t1468 / 2.0_f64;
    let t7847 = t1963 * t1544;
    let t7850 = t207 * t7782;
    (t7787, t7794, t7847, t7850)
}
