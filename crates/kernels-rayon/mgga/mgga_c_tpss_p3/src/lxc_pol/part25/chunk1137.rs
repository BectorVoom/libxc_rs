//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1137/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1137(t12264: f64, t1531: f64, t15361: f64, t15363: f64, t15365: f64, t15411: f64, t15413: f64, t15441: f64, t15446: f64, t15448: f64, t15465: f64, t4120: f64, t4143: f64, t5130: f64, t9471: f64) -> f64 {
    let t15647 = -0.19751673498613801407e-1_f64 * t15441 - t15361 + t15363 - t15365 - t15411 - t15413 - t15446 - t15448 + t15465 + 2.0_f64 * t12264 * t1531 + 2.0_f64 * t4120 * t4143 - 2.0_f64 * t9471 * t5130;
    t15647
}
