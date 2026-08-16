//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 547/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk547(t1293: f64, t409: f64, t1602: f64, t37: f64, t401: f64, t78: f64, t51: f64, t388: f64, t5603: f64, t5607: f64, t1300: f64, t626: f64, t71: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22825 = t409 * t1293;
    let t22826 = t1602 * t22825;
    let t22833 = t37 * t401;
    let t22834 = t22833 * t78;
    let t22837 = t51 * sigma0;
    let t22842 = t388 * t22825;
    let t22849 = t5603 * t5607;
    let t22850 = t1300 * t22849;
    let t22855 = t626 * t71;
    (t22825, t22826, t22833, t22834, t22837, t22842, t22849, t22850, t22855)
}
