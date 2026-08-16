//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1969/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1969(t211: f64, t9644: f64, t138: f64, t785: f64, t9302: f64, t2452: f64, t9720: f64, t11006: f64, t256: f64, t10115: f64, t251: f64, t2410: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39643 = 1.0_f64 / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    let t40688 = t9720 * t2452;
    let t41077 = 1.0_f64 / t11006 / t256;
    let t41117 = t10115 * t251;
    let t41153 = t2410 * t2410;
    (t39643, t40270, t40688, t41077, t41117, t41153)
}
