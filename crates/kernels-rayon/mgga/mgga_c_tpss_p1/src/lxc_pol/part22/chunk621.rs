//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 621/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk621(t30: f64, t259: f64, t379: f64, t2445: f64, t2817: f64, t1991: f64, t1992: f64, t381: f64, t45: f64, t580: f64, t581: f64, t826: f64, t999: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t2818 = piecewise3(t380, t2817, t2445);
    let t2825 = piecewise3(t120, t2445 * t30 / 2.0_f64 + t826 * t580 + t259 * t1991 / 2.0_f64, t2818 * t45 / 2.0_f64 + t999 * t581 + t381 * t1992 / 2.0_f64);
    (t2818, t2825)
}
