//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1006/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1006(t101698: f64, t1888: f64, t232: f64, t6646: f64, t112990: f64, t112995: f64, t121488: f64, t121504: f64, t121524: f64, t121533: f64, t126442: f64, t126446: f64, t126452: f64, t126453: f64, t1510: f64, t812: f64) -> f64 {
    let t127986 = t1888 * t6646 * t101698 * t232;
    let t127990 = -2.0_f64 * t812 * t121488 * t1510 - 0.82246703342411321824e-2_f64 * t121504 - t126442 + t126446 - 0.16449340668482264365e-1_f64 * t127986 + t126452 + t126453 + 0.82246703342411321824e-2_f64 * t121524 + t112990 + t112995 + 0.76763589786250567036e-1_f64 * t121533;
    t127990
}
