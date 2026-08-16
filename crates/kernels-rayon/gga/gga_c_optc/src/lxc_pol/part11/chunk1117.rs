//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1117/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1117(t1133: f64, t45718: f64, t46297: f64, t12489: f64, t4369: f64, t12121: f64, t4310: f64, t45693: f64, t3116: f64, t35165: f64, t5324: f64, t5311: f64, t8446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46832 = t1133 * t45718;
    let t46851 = t1133 * t46297;
    let t46853 = t4369 * t12489;
    let t46886 = t4310 * t12121;
    let t46902 = t1133 * t45693;
    let t46923 = t3116 * t35165 * t5324;
    let t46945 = t8446 * t5311;
    (t46832, t46851, t46853, t46886, t46902, t46923, t46945)
}
