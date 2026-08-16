//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 934/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk934(t1071: f64, t989: f64, t3056: f64, t988: f64, t378: f64, t2258: f64, t606: f64) -> (f64, f64, f64, f64) {
    let t11220 = t989 * t1071;
    let t11223 = t988 * t3056;
    let t11224 = t11223 * t378;
    let t11231 = t606 * t2258;
    (t11220, t11223, t11224, t11231)
}
