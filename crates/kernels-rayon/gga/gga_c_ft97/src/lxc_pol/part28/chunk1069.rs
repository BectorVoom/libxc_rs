//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1069/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1069(t144829: f64, t144832: f64, t144836: f64, t144840: f64, t144844: f64, t144848: f64, t144851: f64, t144855: f64, t144859: f64, t144863: f64, t144866: f64, t144870: f64, t144874: f64, t144878: f64, t144882: f64, t144886: f64) -> f64 {
    let t145807 = -t144829 / 36.0_f64 - t144832 / 3.0_f64 - 20.0_f64 / 3.0_f64 * t144836 + 8.0_f64 / 3.0_f64 * t144840 - t144844 / 36.0_f64 + t144848 / 3.0_f64 - t144851 / 9.0_f64 - t144855 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t144859 - t144863 / 3.0_f64 + 4.0_f64 * t144866 - 2.0_f64 * t144870 - 2.0_f64 / 9.0_f64 * t144874 + t144878 / 2.0_f64 + t144882 / 4.0_f64 + t144886 / 2.0_f64;
    t145807
}
