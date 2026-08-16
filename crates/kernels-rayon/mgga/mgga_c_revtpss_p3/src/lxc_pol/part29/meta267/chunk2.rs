//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1106/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1106(t7058: f64, t7407: f64, t7064: f64, t2061: f64, t886: f64, t7071: f64, t231: f64, t836: f64, t7076: f64, t233: f64, t7398: f64, t1957: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7409 = 0.72280234901709995518e-2_f64 * t7058 * t7407;
    let t7411 = 0.12851425765524037203e-1_f64 * t7064 * t7407;
    let t7414 = t2061 * t886;
    let t7415 = t7071 * t7414;
    let t7419 = t2061 * t836 * t231;
    let t7420 = t7076 * t7419;
    let t7423 = t233 * t7398;
    let t7424 = t1957 * t7423;
    (t7409, t7411, t7415, t7419, t7420, t7423, t7424)
}
