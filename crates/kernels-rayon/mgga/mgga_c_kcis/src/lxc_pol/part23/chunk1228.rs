//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1228/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1228(t1542: f64, t1928: f64, t1394: f64, t7924: f64, t16744: f64, t491: f64, t990: f64, t17279: f64, t27387: f64, t1494: f64, t2242: f64, t15870: f64, t5661: f64) -> (f64, f64, f64, f64, f64) {
    let t98020 = t1542 * t1928;
    let t98022 = t1394 * t98020 * t7924;
    let t98025 = t16744 * t491 * t990;
    let t98030 = t1394 * t27387 * t17279;
    let t98034 = t2242 * t1494;
    let t98036 = t5661 * t98034 * t15870;
    (t98022, t98025, t98030, t98034, t98036)
}
