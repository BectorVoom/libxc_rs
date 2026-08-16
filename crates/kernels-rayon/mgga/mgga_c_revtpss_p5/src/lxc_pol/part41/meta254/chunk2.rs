//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 973/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk973(t670: f64, t8295: f64, t117: f64, t8273: f64, t1459: f64, t1461: f64, t2187: f64, t2189: f64, t572: f64, t573: f64, t8289: f64, t1843: f64, t2178: f64) -> (f64, f64, f64, f64) {
    let t8296 = t8295 * t670;
    let t8299 = t117 * t8273;
    let t8302 = 3.0_f64 * t1459 * t2189 + 3.0_f64 * t1461 * t2187 + 6.0_f64 * t572 * t8296 + 3.0_f64 * t572 * t8299 + t573 * t8289;
    let t8353 = t1843 * t2178;
    (t8296, t8299, t8302, t8353)
}
