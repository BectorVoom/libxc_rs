//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 812/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk812(t2035: f64, t2319: f64, t2095: f64, t22578: f64, t22584: f64, t7170: f64, t1266: f64, t12734: f64, t1393: f64, t1983: f64, t2036: f64, t2040: f64, t2079: f64, t2314: f64, t2323: f64, t2364: f64, t23909: f64, t23918: f64, t23929: f64, t23933: f64, t23938: f64, t3652: f64, t3929: f64, t4034: f64, t510: f64, t652: f64, t672: f64, t7040: f64, t7042: f64, t7050: f64, t7057: f64, t7061: f64, t7166: f64, t9348: f64) -> (f64, f64, f64, f64) {
    let t23941 = t2035 * t2319;
    let t23951 = t2095 * t22578;
    let t23953 = t7170 * t22584;
    let t23956 = -2.0_f64 * t1266 * t7040 - 4.0_f64 * t12734 * t2040 + 2.0_f64 * t1393 * t7166 - t1983 * t23951 + 3.0_f64 * t1983 * t23953 - t2036 * t3652 - 2.0_f64 * t2040 * t9348 + t2079 * t3929 - 4.0_f64 * t2314 * t7050 - 4.0_f64 * t2314 * t7061 - 4.0_f64 * t2323 * t7042 - 2.0_f64 * t2364 * t7042 - 2.0_f64 * t23909 * t652 - 2.0_f64 * t23918 * t652 - 4.0_f64 * t23929 * t652 - 4.0_f64 * t23933 * t652 - 4.0_f64 * t23938 * t672 - 2.0_f64 * t23941 * t510 - 4.0_f64 * t4034 * t7057;
    (t23941, t23951, t23953, t23956)
}
