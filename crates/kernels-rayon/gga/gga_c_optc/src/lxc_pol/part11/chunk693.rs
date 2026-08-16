//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 693/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk693(t2010: f64, t623: f64, t56: f64, t658: f64, t111: f64, t2003: f64, t627: f64, t668: f64, t145: f64, t128: f64, t2155: f64, t131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6941 = t623 * t2010;
    let t6944 = t56 * t658;
    let t6945 = t111 * t6944;
    let t6956 = t2003 * t627;
    let t6975 = t668 * t668;
    let t6976 = 1.0_f64 / t6975;
    let t6977 = t145 * t6976;
    let t6990 = 1.0_f64 / t2155 / t128;
    let t6991 = t6990 * t131;
    (t6941, t6944, t6945, t6956, t6975, t6976, t6977, t6990, t6991)
}
