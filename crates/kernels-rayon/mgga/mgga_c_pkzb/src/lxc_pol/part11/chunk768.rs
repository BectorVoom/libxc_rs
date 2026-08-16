//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 768/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk768(t5093: f64, t972: f64, t1642: f64, t8: f64, t1003: f64, t5106: f64, t1651: f64, t2557: f64, t46: f64, t552: f64, t1667: f64, t2620: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6767 = t5093 * t972;
    let t6770 = t1642 * t8;
    let t6782 = t5106 * t1003;
    let t6785 = t1651 * t8;
    let t6801 = t2557 * t46;
    let t6803 = 0.36622894612013090108e-3_f64 * t6801 * t552;
    let t6804 = t2620 * t1667;
    (t6767, t6770, t6782, t6785, t6801, t6803, t6804)
}
