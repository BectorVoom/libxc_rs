//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1242/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1242(t13888: f64, t3306: f64, t353: f64, t859: f64, t14404: f64, t19906: f64, t13917: f64, t3258: f64, t51021: f64, t51023: f64, t1114: f64, t50942: f64) -> (f64, f64, f64, f64) {
    let t53220 = t859 * t353 * t13888 * t3306;
    let t53224 = 7.0_f64 / 72.0_f64 * t19906 * t14404;
    let t53227 = t13917 * t51021 * t3258 * t51023;
    let t53229 = t1114 * t50942;
    (t53220, t53224, t53227, t53229)
}
