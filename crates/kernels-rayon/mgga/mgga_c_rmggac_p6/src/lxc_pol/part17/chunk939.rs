//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 939/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk939(t2191: f64, t9935: f64, t1986: f64, t6592: f64, t675: f64, t1743: f64, t352: f64, t1756: f64, t7567: f64, t2160: f64, t638: f64, t9754: f64) -> (f64, f64, f64, f64, f64) {
    let t45614 = t2191 * t9935;
    let t45617 = t675 * t1986 * t6592;
    let t45622 = t1743 * t352;
    let t45626 = t7567 * t1756;
    let t45630 = t638 * t2160 * t9754;
    (t45614, t45617, t45622, t45626, t45630)
}
