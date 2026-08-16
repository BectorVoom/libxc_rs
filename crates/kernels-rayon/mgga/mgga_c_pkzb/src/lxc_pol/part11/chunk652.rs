//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 652/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk652(t12: f64, t3366: f64, t1151: f64, t1153: f64, t318: f64, t319: f64, t3706: f64, t201: f64, t398: f64, t326: f64, t2179: f64, t3371: f64, t3374: f64, t821: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t3710 = piecewise3(t84, 0.0_f64, t3366);
    let t3714 = piecewise3(t203, 0.0_f64, t3706 * t319 / 2.0_f64 + t1151 * t1153 + t318 * t3710 / 2.0_f64);
    let t3715 = t201 * t3714;
    let t3718 = 1.0_f64 / t398;
    let t3719 = t326 * t3718;
    let t3725 = t2179 * t3371;
    let t3727 = t821 * t3374;
    (t3710, t3715, t3719, t3725, t3727)
}
