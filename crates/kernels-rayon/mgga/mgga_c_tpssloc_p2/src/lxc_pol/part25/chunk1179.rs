//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1179/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1179(t109: f64, t81437: f64, t81440: f64, t81443: f64, t81445: f64, t81447: f64, t81450: f64, t81452: f64, t112: f64, t24447: f64, t111: f64, t24007: f64, t12492: f64, t12504: f64, t1266: f64, t12734: f64, t12823: f64, t1983: f64, t2040: f64, t2075: f64, t2079: f64, t2095: f64, t22574: f64, t22578: f64, t22584: f64, t2320: f64, t2323: f64, t23917: f64, t23918: f64, t23929: f64, t23938: f64, t24175: f64, t26558: f64, t3652: f64, t39235: f64, t3929: f64, t4034: f64, t510: f64, t55183: f64, t652: f64, t672: f64, t7040: f64, t7042: f64, t7050: f64, t7056: f64, t7057: f64, t7156: f64, t7166: f64, t7217: f64, t83911: f64, t9347: f64) -> (f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t84036 = 308.0_f64 / 27.0_f64 * t81437;
    let t84044 = piecewise3(t110, 0.0_f64, -t84036 - 22.0_f64 / 3.0_f64 * t81440 - 4.0_f64 * t81443 + 2.0_f64 * t81445 - 3.0_f64 / 2.0_f64 * t81447 + 3.0_f64 / 2.0_f64 * t81450 - t81452 / 4.0_f64);
    let t84078 = t24447 * t112;
    let t84097 = t24007 * t111;
    let t84130 = -6.0_f64 * t12823 * t7057 - 12.0_f64 * t12734 * t7050 - 3.0_f64 * t1983 * t7217 * t22578 + 3.0_f64 * t7166 * t3929 + t2079 * t12492 - t1983 * t2095 * t83911 - 6.0_f64 * t84097 * t672 - t9347 * t2075 - 3.0_f64 * t7040 * t3652 + 18.0_f64 * t22574 * t26558 * t55183 - 6.0_f64 * t652 * t3652 * t7056 - 6.0_f64 * t4034 * t23918 - 6.0_f64 * t652 * t1266 * t23917 - 2.0_f64 * t39235 * t2040 - 2.0_f64 * t652 * t510 * t84044 + 9.0_f64 * t1983 * t24175 * t22584 - 6.0_f64 * t2320 * t7156 - 12.0_f64 * t23938 * t2323 - 6.0_f64 * t7042 * t12504 - 12.0_f64 * t4034 * t23929;
    (t84044, t84078, t84097, t84130)
}
