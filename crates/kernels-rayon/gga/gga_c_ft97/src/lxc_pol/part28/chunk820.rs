//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 820/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk820(t2179: f64, t33080: f64, t574: f64, t609: f64, t7400: f64, t9439: f64, t144: f64, t1384: f64, t5968: f64, t32895: f64, t32922: f64, t32892: f64, t32902: f64, t32910: f64, t32915: f64, t32919: f64, t32927: f64, t32931: f64, t32935: f64, t32940: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33082 = t574 * t2179 * t33080;
    let t33085 = t7400 * t609;
    let t33086 = t9439 * t33085;
    let t33087 = t144 * t33086;
    let t33090 = t1384 * t5968;
    let t33091 = t2179 * t33090;
    let t33092 = t144 * t33091;
    let t33096 = 2.0_f64 / 9.0_f64 * t32895;
    let t33101 = t32922 / 9.0_f64;
    let t33105 = t32892 / 2.0_f64 + t33096 + 2.0_f64 / 9.0_f64 * t32902 + 4.0_f64 / 3.0_f64 * t32910 - 2.0_f64 / 3.0_f64 * t32915 - t32919 / 6.0_f64 - t33101 - t32927 / 9.0_f64 - t32931 + 2.0_f64 / 3.0_f64 * t32935 + t32940 / 12.0_f64;
    (t33082, t33085, t33086, t33087, t33090, t33091, t33092, t33096, t33101, t33105)
}
