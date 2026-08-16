//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1029/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1029(t11351: f64, t11368: f64, t405: f64, t921: f64, t758: f64, t10059: f64, t10061: f64, t10081: f64, t10132: f64, t10136: f64, t10197: f64, t11338: f64, t11342: f64, t11348: f64, t1230: f64, t3174: f64, t3877: f64, t407: f64, t6379: f64, t6459: f64, t8360: f64, t918: f64) -> (f64, f64, f64) {
    let t11369 = t11351 + t11368;
    let t11371 = t405 * t11369 * t921;
    let t11372 = t758 * t11371;
    let t11381 = -0.45732285992607719436e-2_f64 * t10059 + 0.14481890564325777821e-1_f64 * t10061 - 0.17149607247227894789e-2_f64 * t10081 + t6379 - 0.53100265402527852012e-1_f64 * t11338 * t407 + t3174 * t11342 / 16.0_f64 + 0.21437009059034868486e-3_f64 * t6459 * t11348 + 0.21437009059034868486e-3_f64 * t918 * t11372 + 0.21722835846488666732e-1_f64 * t10197 * t1230 + 0.34299214494455789577e-2_f64 * t8360 * t3877 + t10132 / 48.0_f64 - t10136 / 96.0_f64;
    (t11369, t11371, t11381)
}
