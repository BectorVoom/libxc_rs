//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1294/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1294(t2371: f64, t26123: f64, t572: f64, t1459: f64, t26130: f64, t4158: f64, t7331: f64, t7334: f64, t13232: f64, t2042: f64, t28264: f64, t2327: f64, t7002: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95143 = 18.0_f64 * t572 * t26123 * t2371;
    let t95147 = 9.0_f64 * t1459 * t26130;
    let t95149 = 18.0_f64 * t4158 * t7331;
    let t95153 = 9.0_f64 * t4158 * t7334;
    let t95157 = 3.0_f64 * t13232 * t2042;
    let t95160 = 18.0_f64 * t572 * t28264 * t2371;
    let t95163 = 18.0_f64 * t572 * t2327 * t7002;
    (t95143, t95147, t95149, t95153, t95157, t95160, t95163)
}
