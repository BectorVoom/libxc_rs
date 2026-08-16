//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1478/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1478(t3: f64, t31582: f64, t2178: f64, t5883: f64, t1518: f64, t31370: f64, t5920: f64, t8295: f64, t117: f64, t31555: f64, t1916: f64, t1918: f64, t2187: f64, t2189: f64, t572: f64, t573: f64, t6941: f64, t6945: f64, t6948: f64, t8377: f64, t8383: f64, t8386: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31583 = t3 * t31582;
    let t31593 = param_d * t31582;
    let t31607 = t5883 * t2178;
    let t31610 = t31370 * t1518;
    let t31613 = t8295 * t5920;
    let t31616 = t117 * t31555;
    let t31619 = 12.0_f64 * t1916 * t8383 + 6.0_f64 * t1916 * t8386 + 6.0_f64 * t1918 * t8377 + 6.0_f64 * t2187 * t6945 + 3.0_f64 * t2187 * t6948 + 3.0_f64 * t2189 * t6941 + t31593 * t573 + 6.0_f64 * t31607 * t572 + 12.0_f64 * t31610 * t572 + 6.0_f64 * t31613 * t572 + 3.0_f64 * t31616 * t572;
    (t31583, t31593, t31607, t31610, t31613, t31616, t31619)
}
