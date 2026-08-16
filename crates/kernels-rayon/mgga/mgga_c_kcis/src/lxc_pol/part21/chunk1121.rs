//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1121/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1121(t2201: f64, t3668: f64, t2205: f64, t3699: f64, t26867: f64, t26870: f64, t26873: f64, t26874: f64, t26875: f64, t26879: f64, t26882: f64, t26888: f64, t3669: f64, t3670: f64, t7809: f64) -> (f64, f64, f64) {
    let t27141 = t2201 * t3668;
    let t27144 = t2205 * t3699;
    let t27147 = 2.0_f64 * t27141 * t3670 + 2.0_f64 * t27144 * t3669 - t3699 * t7809 - t26867 + t26870 - t26873 + t26874 + t26875 + t26879 + t26882 - t26888;
    (t27141, t27144, t27147)
}
