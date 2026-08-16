//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1418/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1418(t11328: f64, t698: f64, t2439: f64, t2915: f64, t2909: f64, t11345: f64, t41246: f64, t41250: f64, t41255: f64, t41260: f64, t41265: f64, t41267: f64, t41273: f64, t41275: f64, t41279: f64, t41281: f64) -> (f64, f64, f64, f64, f64) {
    let t41283 = t698 * t11328;
    let t41285 = t2439 * t2915;
    let t41287 = t2439 * t2909;
    let t41289 = t698 * t11345;
    let t41291 = t41246 - 0.22076e0_f64 * t41250 + 0.66228e0_f64 * t41255 - 0.11038e0_f64 * t41260 + 0.99342e0_f64 * t41265 - 0.132456e1_f64 * t41267 + 0.44152e0_f64 * t41273 + 0.132456e1_f64 * t41275 - 0.99342e0_f64 * t41279 + 0.11038e1_f64 * t41281 - 0.44152e0_f64 * t41283 - 0.5519e0_f64 * t41285 - 0.18396666666666666667e0_f64 * t41287 + 0.22076e0_f64 * t41289;
    (t41283, t41285, t41287, t41289, t41291)
}
