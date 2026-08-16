//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1076/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1076(t1609: f64, t1615: f64, t2462: f64, t2468: f64, t2188: f64, t286: f64, t442: f64, t7592: f64, t291: f64, t7875: f64, t103: f64, t332: f64, t7877: f64, t818: f64) -> (f64, f64, f64, f64, f64) {
    let t15430 = t1609 * t1615;
    let t15436 = t2462 * t2468;
    let t15473 = t7592 * t2188 * t286 * t442;
    let t15479 = t291 * t7875;
    let t15483 = t15479 * t332 * t818 * t7877 * t103;
    (t15430, t15436, t15473, t15479, t15483)
}
