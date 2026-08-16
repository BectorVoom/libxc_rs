//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 618/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk618(t263: f64, t6837: f64, t684: f64, t2354: f64, t10157: f64, t3837: f64, t6003: f64, t1091: f64, t24240: f64, t24245: f64, t1402: f64, t3051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27991 = t6837 * t263;
    let t27992 = t27991 * t684;
    let t27993 = t2354 * t27992;
    let t27997 = t10157 * t6003 * t3837;
    let t28001 = t24240 * t1091;
    let t28002 = t2354 * t28001;
    let t28006 = t2354 * t24245 * t1091;
    let t28010 = t1402 * t3051;
    (t27991, t27992, t27993, t27997, t28001, t28002, t28006, t28010)
}
