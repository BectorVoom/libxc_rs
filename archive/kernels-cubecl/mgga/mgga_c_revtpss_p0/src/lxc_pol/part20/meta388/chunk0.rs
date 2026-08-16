//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1418/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1418<F: Float>(t11328: F, t698: F, t2439: F, t2915: F, t2909: F, t11345: F, t41246: F, t41250: F, t41255: F, t41260: F, t41265: F, t41267: F, t41273: F, t41275: F, t41279: F, t41281: F) -> (F, F, F, F, F) {
    let t41283 = t698 * t11328;
    let t41285 = t2439 * t2915;
    let t41287 = t2439 * t2909;
    let t41289 = t698 * t11345;
    let t41291 = t41246 - F::cast_from(0.22076e0_f64) * t41250 + F::cast_from(0.66228e0_f64) * t41255 - F::cast_from(0.11038e0_f64) * t41260 + F::cast_from(0.99342e0_f64) * t41265 - F::cast_from(0.132456e1_f64) * t41267 + F::cast_from(0.44152e0_f64) * t41273 + F::cast_from(0.132456e1_f64) * t41275 - F::cast_from(0.99342e0_f64) * t41279 + F::cast_from(0.11038e1_f64) * t41281 - F::cast_from(0.44152e0_f64) * t41283 - F::cast_from(0.5519e0_f64) * t41285 - F::cast_from(0.18396666666666666667e0_f64) * t41287 + F::cast_from(0.22076e0_f64) * t41289;
    (t41283, t41285, t41287, t41289, t41291)
}
