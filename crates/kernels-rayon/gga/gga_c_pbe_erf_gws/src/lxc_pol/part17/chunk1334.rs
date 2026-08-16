//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1334/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1334(t22393: f64, t2409: f64, t3959: f64, t898: f64, t911: f64, t3973: f64, t3972: f64, t53800: f64, t8884: f64, t13953: f64, t14787: f64, t13796: f64, t13859: f64, t2171: f64, t52921: f64) -> (f64, f64, f64, f64) {
    let t54496 = t3959 * t2409 * t22393;
    let t54498 = t911 * t898;
    let t54499 = t3973 * t54498;
    let t54502 = t3972 * t54499 * t8884 * t53800;
    let t54504 = t13953 * t14787;
    let t54505 = 7.0_f64 / 144.0_f64 * t54504;
    let t54508 = t13859 * t13796 * t52921 * t2171;
    (t54496, t54502, t54505, t54508)
}
