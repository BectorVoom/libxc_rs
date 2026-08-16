//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1228/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1228(t22074: f64, t22079: f64, t22098: f64, t22103: f64, t22107: f64, t22111: f64, t22115: f64, t55882: f64, t55883: f64, t55884: f64, t55944: f64, t55977: f64, t55980: f64) -> f64 {
    let t56256 = t55882 - t55883 - t55884 - t55944 - t55977 + t22074 - t55980 + t22079 - t22098 - t22103 + t22107 + t22111 + t22115;
    t56256
}
