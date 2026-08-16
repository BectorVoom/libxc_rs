//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1292/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1292(t63977: f64, t63990: f64, t1395: f64, t18770: f64, t20447: f64, t219: f64, t1805: f64, t8275: f64, t1219: f64, t6419: f64, t10085: f64, t1838: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t66429 = 35.0_f64 / 144.0_f64 * t63977;
    let t66434 = 7.0_f64 / 12.0_f64 * t63990;
    let t66480 = t18770 * t1395;
    let t66525 = t20447 * t219;
    let t66559 = t8275 * t1805;
    let t66970 = t1219 * t6419;
    let t67006 = t10085 * t1838;
    (t66429, t66434, t66480, t66525, t66559, t66970, t67006)
}
