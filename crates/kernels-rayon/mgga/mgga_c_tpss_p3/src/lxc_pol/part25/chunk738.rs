//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 738/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk738(t4834: f64, t835: f64, t128: f64, t2454: f64, t3746: f64, t4828: f64, t4832: f64, t285: f64, t1425: f64, t3765: f64, t1424: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4835 = t835 * t4834;
    let t4836 = t128 * t4835;
    let t4838 = t2454 + 0.11872222222222222222e-1_f64 * t3746 - 0.11872222222222222222e-1_f64 * t4828 + 0.35616666666666666666e-1_f64 * t4832 - 0.17808333333333333333e-1_f64 * t4836;
    let t4840 = 0.621814e-1_f64 * t4838 * t285;
    let t4842 = 2.0_f64 * t3765 * t1425;
    let t4843 = t1424 * t1424;
    let t4844 = t4843 * t866;
    (t4835, t4836, t4838, t4840, t4842, t4843, t4844)
}
