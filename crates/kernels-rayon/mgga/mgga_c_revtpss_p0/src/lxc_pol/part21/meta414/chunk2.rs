//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1889/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1889(t13312: f64, t48: f64, t10368: f64, t1469: f64, t2251: f64, t2282: f64, t4186: f64, t606: f64, t2258: f64, t4210: f64, t60: f64, t10379: f64, t13299: f64, t13303: f64, t13306: f64, t1474: f64, t1480: f64, t2270: f64, t2283: f64, t2286: f64, t4202: f64, t4205: f64, t44: f64, t56: f64, t614: f64) -> (f64, f64, f64) {
    let t13313 = t48 * t13312;
    let t13321 = t10368 * t1469 * t2251;
    let t13324 = t2282 * t4186;
    let t13325 = t13324 * t606;
    let t13328 = t4210 * t2258;
    let t13331 = t60 * t13312;
    let t13334 = 220.0_f64 / 27.0_f64 * t2270 * t1474 - 40.0_f64 / 27.0_f64 * t614 * t4202 - 40.0_f64 / 9.0_f64 * t614 * t4205 - 5.0_f64 / 108.0_f64 * t44 * t13299 + 5.0_f64 / 9.0_f64 * t44 * t13303 + 5.0_f64 / 18.0_f64 * t44 * t13306 + 5.0_f64 / 6.0_f64 * t44 * t13313 - 20.0_f64 / 27.0_f64 * t1480 * t2283 + 20.0_f64 / 9.0_f64 * t1480 * t2286 + 5.0_f64 / 108.0_f64 * t56 * t13321 + 5.0_f64 / 9.0_f64 * t56 * t13325 + 5.0_f64 / 18.0_f64 * t56 * t13328 - 5.0_f64 / 6.0_f64 * t56 * t13331 + t10379;
    (t13313, t13324, t13334)
}
