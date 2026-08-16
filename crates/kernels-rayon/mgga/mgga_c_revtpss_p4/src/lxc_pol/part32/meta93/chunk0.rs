//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 574/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk574(t265: f64, t393: f64, t2071: f64, t30: f64, t207: f64, t2070: f64, t198: f64, t892: f64) -> (f64, f64, f64, f64) {
    let t394 = t265 < t393;
    let t2072 = t2071 * t30;
    let t2075 = t207 * t2070;
    let t2077 = t198 * t2075 * t892;
    let t2078 = piecewise3(t394, 0.0_f64, t2077);
    (t2072, t2075, t2077, t2078)
}
