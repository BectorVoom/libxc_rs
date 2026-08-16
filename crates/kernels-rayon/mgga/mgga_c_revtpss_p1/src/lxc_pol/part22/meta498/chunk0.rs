//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2230/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2230(t1086: f64, t4743: f64, t1089: f64, t15920: f64, t16076: f64, t12073: f64, t1651: f64, t1082: f64, t16152: f64, t15837: f64, t3075: f64, t4975: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16381 = t4743 * t1086;
    let t16390 = t15920 * t1089;
    let t16393 = t16076 * t1089;
    let t16396 = t12073 * t1651;
    let t16399 = t1082 * t16152;
    let t16402 = t1082 * t15837;
    let t16405 = t4975 * t3075;
    (t16381, t16390, t16393, t16396, t16399, t16402, t16405)
}
