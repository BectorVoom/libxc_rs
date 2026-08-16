//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1079/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1079(t102: f64, t1303: f64, t1946: f64, t1609: f64, t1615: f64, t2462: f64, t2468: f64, t2188: f64, t286: f64, t442: f64, t7592: f64, t291: f64, t7875: f64) -> (f64, f64, f64, f64, f64) {
    let t15358 = t1946 * t102 * t1303;
    let t15430 = t1609 * t1615;
    let t15436 = t2462 * t2468;
    let t15473 = t7592 * t2188 * t286 * t442;
    let t15479 = t291 * t7875;
    (t15358, t15430, t15436, t15473, t15479)
}
