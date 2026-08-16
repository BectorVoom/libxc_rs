//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 851/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk851(t6547: f64, t8357: f64, t1902: f64, t234: f64, t794: f64, t8356: f64, t6562: f64, t6585: f64, t8339: f64, t6600: f64, t6599: f64, t240: f64, t241: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30675 = 0.38381794893125283518e-1_f64 * t6547 * t8357;
    let t30676 = t234 * t1902;
    let t30681 = t794 * t8356;
    let t30683 = 0.82246703342411321825e-2_f64 * t6562 * t30681;
    let t30697 = t6585 * t8339;
    let t30703 = t6600 * t8339;
    let t30704 = t6599 * t30703;
    let t30713 = t814 * t240 * t241;
    (t30675, t30676, t30681, t30683, t30697, t30703, t30704, t30713)
}
