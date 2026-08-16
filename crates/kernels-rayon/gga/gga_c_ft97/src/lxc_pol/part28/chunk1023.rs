//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1023/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1023(t1871: f64, t25893: f64, t25894: f64, t32094: f64, t22952: f64, t25888: f64, t8411: f64, t144829: f64, t144832: f64, t144836: f64, t144840: f64, t144844: f64, t144848: f64, t144851: f64, t144855: f64, t144859: f64, t144863: f64, t144866: f64, t144870: f64, t144874: f64, t144878: f64) -> (f64, f64, f64) {
    let t144882 = t25893 * t1871 * t32094 * t25894;
    let t144886 = t22952 * t8411 * t32094 * t25888;
    let t144888 = -t144829 / 12.0_f64 - t144832 - 20.0_f64 * t144836 + 8.0_f64 * t144840 - t144844 / 12.0_f64 + t144848 - t144851 / 3.0_f64 - t144855 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t144859 - t144863 + 12.0_f64 * t144866 - 6.0_f64 * t144870 - 2.0_f64 / 3.0_f64 * t144874 + 3.0_f64 / 2.0_f64 * t144878 + 3.0_f64 / 4.0_f64 * t144882 + 3.0_f64 / 2.0_f64 * t144886;
    (t144882, t144886, t144888)
}
