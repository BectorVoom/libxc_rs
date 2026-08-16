//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1722/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1722(t22035: f64, t22065: f64, t22105: f64, t22140: f64, t22153: f64, t22176: f64, t22284: f64, t22304: f64, t6862: f64, t72: f64, t686: f64, t10023: f64) -> (f64, f64) {
    let t22307 = t22035 + t22065 + t22105 + t22140 + t22153 + t22176 + t22284 + t22304;
    let t22314 = t6862 * t72;
    let t22315 = t22314 * t686;
    let t22316 = t10023 * t22315;
    (t22307, t22316)
}
