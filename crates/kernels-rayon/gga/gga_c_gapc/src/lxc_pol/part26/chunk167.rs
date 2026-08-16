//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 167/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk167(t153: f64, t583: f64, t181: f64, t118: f64, t6: f64, t481: f64, t169: f64, t173: f64, t435: f64, t122: f64, t188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t584 = t153 * t583;
    let t585 = t181 * t584;
    let t588 = t118 * t6;
    let t589 = t588 * t481;
    let t590 = t169 * t589;
    let t591 = t435 * t173;
    let t594 = t118 * t122;
    let t595 = t594 * t188;
    let t596 = t169 * t595;
    (t584, t585, t588, t589, t590, t591, t594, t595, t596)
}
