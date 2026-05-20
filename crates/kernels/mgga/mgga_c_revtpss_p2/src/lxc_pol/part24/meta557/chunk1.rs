//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1666/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1666<F: Float>(t88202: F, t923: F, t141: F, t2908: F, t88124: F, t88087: F, t930: F, t52128: F, t63453: F, t63459: F, t63464: F, t63533: F, t63538: F, t63545: F, t77559: F, t77561: F, t77806: F, t77858: F) -> (F, F, F, F) {
    let t88252 = t923 * t88202;
    let t88257 = t141 * t2908 * t88124;
    let t88260 = t141 * t930 * t88087;
    let t88262 = F::new(0.22076e0) * t77806 + F::cast_from(0.98115555555555555556e0_f64) * t52128 - F::cast_from(0.53675555555555555556e0_f64) * t63453 + F::cast_from(0.16102666666666666667e1_f64) * t63459 - F::cast_from(0.18396666666666666667e0_f64) * t63533 + F::new(0.11038e1) * t63538 - F::new(0.5519e0) * t63545 + F::cast_from(0.80513333333333333333e0_f64) * t77559 - F::new(0.24154e1) * t77561 + F::new(0.16504875e0) * t88252 - F::cast_from(0.80513333333333333336e0_f64) * t63464 + F::new(0.22076e0) * t77858 + F::new(0.99342e0) * t88257 - F::new(0.298026e1) * t88260;
    (t88252, t88257, t88260, t88262)
}
