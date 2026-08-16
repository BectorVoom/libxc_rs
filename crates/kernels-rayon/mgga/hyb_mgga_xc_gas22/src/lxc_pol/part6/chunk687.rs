//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 687/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk687(t7: f64, t132: f64, t1325: f64, t1382: f64, t220: f64, t291: f64, t3294: f64, t3448: f64, t771: f64, t861: f64, t3293: f64, t1392: f64, t222: f64, t568: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t133 = t132 <= zeta_threshold;
    let t3452 = piecewise3(t9, 0.0_f64, t1325 * t861 / 2.0_f64 + t771 * t1382 / 2.0_f64 + t220 * t3448 / 2.0_f64 + t3294 * t291 / 2.0_f64);
    let t3455 = piecewise3(t133, 0.0_f64, -t3293);
    let t3461 = t222 * t568 * t1392;
    (t3452, t3455, t3461)
}
