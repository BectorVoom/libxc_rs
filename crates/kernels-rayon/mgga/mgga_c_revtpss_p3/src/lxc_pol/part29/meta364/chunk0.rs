//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1310/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1310(t2371: f64, t93: f64, t1514: f64, t2289: f64, t4264: f64, t625: f64, t4288: f64, t10208: f64, t1513: f64, t2340: f64, t2339: f64, t4287: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13440 = t93 * t2371;
    let t13448 = t2289 * t1514;
    let t13451 = 4.0_f64 / 3.0_f64 * t625 * t4264;
    let t13453 = 2.0_f64 / 3.0_f64 * t625 * t4288;
    let t13455 = t10208 * t1513 * t2340;
    let t13458 = t2339 * t4287;
    (t13440, t13448, t13451, t13453, t13455, t13458)
}
