//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 864/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk864(t12309: f64, t3074: f64, t1529: f64, t862: f64, t3089: f64, t945: f64, t1240: f64, t3036: f64, t3037: f64, t1160: f64, t407: f64, t441: f64, t879: f64) -> (f64, f64, f64, f64, f64) {
    let t12310 = t12309 * t3074;
    let t12313 = t862 * t1529;
    let t12315 = t12313 * t3089 * t945;
    let t12318 = t3036 * t1240 * t3037;
    let t12326 = t1160 * t441 * t879 * t407;
    (t12310, t12313, t12315, t12318, t12326)
}
