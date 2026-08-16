//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 498/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk498(t3799: f64, t706: f64, t1123: f64, t173: f64, t701: f64, t2440: f64, t420: f64, t3691: f64, t2320: f64, t3700: f64, t18: f64, t704: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3800 = t3799 * t706;
    let t3803 = t173 * t1123;
    let t3804 = t701 * t3803;
    let t3806 = t420 * t2440;
    let t3807 = t3806 * t3691;
    let t3808 = t701 * t3807;
    let t3810 = t2320 * t3700;
    let t3811 = t701 * t3810;
    let t3813 = t704 * t18;
    (t3800, t3803, t3804, t3806, t3807, t3808, t3810, t3811, t3813)
}
