//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1206/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1206(t1094: f64, t6680: f64, t1172: f64, t19619: f64, t5047: f64, t5046: f64, t14785: f64, t5073: f64, t19856: f64, t3338: f64, t10526: f64, t6690: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t20155 = t6680 * t1094;
    let t20156 = t20155 * sigma0;
    let t20157 = t20156 * t1172;
    let t20159 = t5047 * t19619;
    let t20160 = t5046 * t20159;
    let t20162 = t14785 * t5073;
    let t20164 = t3338 * t19856;
    let t20165 = t5046 * t20164;
    let t20167 = t10526 * t6690;
    (t20157, t20160, t20162, t20165, t20167)
}
