//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1187/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1187(t12734: f64, t12823: f64, t1393: f64, t1983: f64, t2040: f64, t2075: f64, t2096: f64, t22607: f64, t2314: f64, t2363: f64, t23918: f64, t23929: f64, t23933: f64, t23951: f64, t24026: f64, t24166: f64, t24176: f64, t24442: f64, t4034: f64, t45637: f64, t45640: f64, t510: f64, t574: f64, t652: f64, t6876: f64, t6999: f64, t7050: f64, t7057: f64, t7156: f64, t7170: f64, t7220: f64, t83863: f64, t83904: f64, t84291: f64, t84298: f64, t9416: f64) -> f64 {
    let t84322 = -6.0_f64 * t2314 * t24442 - 6.0_f64 * t4034 * t24442 - 6.0_f64 * t652 * t7156 * t2363 + 3.0_f64 * t1983 * t7170 * t83863 + t83904 * t2096 - 3.0_f64 * t6876 * t23951 - 3.0_f64 * t1983 * t24166 * t6999 + 3.0_f64 * t24026 * t1393 + t84298 * t574 - 12.0_f64 * t2314 * t23933 - t84291 * t510 + 18.0_f64 * t6876 * t24176 - 6.0_f64 * t2314 * t23918 - 6.0_f64 * t45637 * t2040 - 2.0_f64 * t45640 * t2040 - 6.0_f64 * t12823 * t7050 - 2.0_f64 * t652 * t2075 * t9416 - 12.0_f64 * t12734 * t7057 - 12.0_f64 * t2314 * t23929 - 3.0_f64 * t22607 * t7220;
    t84322
}
