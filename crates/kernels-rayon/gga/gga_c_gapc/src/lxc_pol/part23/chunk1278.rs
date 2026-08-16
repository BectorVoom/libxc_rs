//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1278/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1278(t11673: f64, t128: f64, t22970: f64, t24499: f64, t10346: f64, t11683: f64, t23305: f64, t2440: f64, t22657: f64, t2456: f64, t11636: f64, t11684: f64, t6940: f64) -> (f64, f64, f64, f64) {
    let t35772 = t11673 * t22970 * t128 * t24499;
    let t35776 = t10346 * t23305 * t11683 * t2440;
    let t35780 = t10346 * t22657 * t11683 * t2456;
    let t35783 = t11636 * t6940 * t11684;
    (t35772, t35776, t35780, t35783)
}
