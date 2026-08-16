//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1259/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1259(t10346: f64, t11683: f64, t22657: f64, t2456: f64, t11636: f64, t11684: f64, t6940: f64, t10123: f64, t10243: f64, t2531: f64, t329: f64, t827: f64) -> (f64, f64, f64) {
    let t35780 = t10346 * t22657 * t11683 * t2456;
    let t35783 = t11636 * t6940 * t11684;
    let t35788 = t10243 * t827 * t10123 * t329 * t2531;
    (t35780, t35783, t35788)
}
