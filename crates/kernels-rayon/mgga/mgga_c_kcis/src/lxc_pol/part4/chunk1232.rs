//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1232/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1232(t1464: f64, t15900: f64, t3805: f64, t5632: f64, t1395: f64, t1394: f64, t1017: f64, t541: f64, t86: f64, t2011: f64, t4134: f64, t4129: f64) -> (f64, f64, f64, f64) {
    let t15901 = t1464 * t15900;
    let t15903 = t5632 * t3805;
    let t15904 = t1395 * t15903;
    let t15905 = t1394 * t15904;
    let t15909 = t86 * t1017 * t541;
    let t15910 = t4134 * t2011;
    let t15911 = t15910 * t4129;
    (t15901, t15905, t15909, t15911)
}
