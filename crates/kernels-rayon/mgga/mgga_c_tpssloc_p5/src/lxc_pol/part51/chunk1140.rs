//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1140/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1140(t214: f64, t30689: f64, t1880: f64, t6585: f64, t8339: f64, t1894: f64, t59: f64, t776: f64, t6591: f64, t6600: f64, t6599: f64, t6612: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30690 = t214 * t30689;
    let t30692 = 0.16449340668482264365e-1_f64 * t1880 * t30690;
    let t30697 = t6585 * t8339;
    let t30700 = t1894 * t59 * t776;
    let t30701 = t6591 * t30700;
    let t30703 = t6600 * t8339;
    let t30704 = t6599 * t30703;
    let t30706 = t6612 * t829;
    (t30690, t30692, t30697, t30700, t30701, t30703, t30704, t30706)
}
