//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1977/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1977(t30950: f64, t30973: f64, t3: f64, t1918: f64, t2170: f64, t30180: f64, t30182: f64, t30184: f64, t30187: f64, t30190: f64, t30193: f64, t30196: f64, t573: f64, t6945: f64, t6948: f64, t8245: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t30974 = t30950 + t30973;
    let t30975 = t3 * t30974;
    let t30985 = param_d * t30974;
    let t30993 = 6.0_f64 * t1918 * t8245 + 6.0_f64 * t2170 * t6945 + 3.0_f64 * t2170 * t6948 + t30985 * t573 + t30180 + t30182 + t30184 + t30187 + t30190 + t30193 + t30196;
    (t30974, t30975, t30985, t30993)
}
