//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 864/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk864(t1514: f64, t2289: f64, t1857: f64, t3857: f64, t2516: f64, t5571: f64, t1320: f64, t5569: f64, t2626: f64, t1856: f64, t2608: f64, t512: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13448 = t2289 * t1514;
    let t13584 = t3857 * t1857;
    let t13611 = t5571 * t2516;
    let t13621 = t1320 * t5569;
    let t13630 = t5571 * t2626;
    let t13632 = t1856 * t2608;
    let t13633 = t512 * t13632;
    (t13448, t13584, t13611, t13621, t13630, t13633)
}
