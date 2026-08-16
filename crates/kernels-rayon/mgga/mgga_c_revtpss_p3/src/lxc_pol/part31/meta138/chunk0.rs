//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 740/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk740(t1175: f64, t300: f64, t3356: f64, t1203: f64, t1208: f64) -> (f64, f64, f64) {
    let t3531 = t300 * t1175;
    let t3546 = 0.11111111111111111111e-1_f64 * t3356;
    let t3555 = t1203 * t1208;
    (t3531, t3546, t3555)
}
