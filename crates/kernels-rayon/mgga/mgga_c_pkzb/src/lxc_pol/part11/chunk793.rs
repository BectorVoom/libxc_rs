//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 793/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk793(t4803: f64, t204: f64, t3026: f64, t648: f64, t1003: f64, t6097: f64, t2179: f64, t8: f64, t1180: f64, t1878: f64, t218: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7907 = 6.0_f64 * t4803;
    let t7930 = t204 * t648 * t3026;
    let t7931 = 0.59793333333333333334e0_f64 * t7930;
    let t7932 = t6097 * t1003;
    let t7935 = t2179 * t8;
    let t7950 = t218 * t1878 * t1180;
    (t7907, t7930, t7931, t7932, t7935, t7950)
}
