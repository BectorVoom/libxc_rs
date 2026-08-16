//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1494/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1494(t109: f64, t45509: f64, t11968: f64, t11972: f64, t12504: f64, t12507: f64, t1266: f64, t1268: f64, t12734: f64, t2312: f64, t2314: f64, t2323: f64, t2363: f64, t2364: f64, t3652: f64, t39223: f64, t39231: f64, t39235: f64, t4034: f64, t45408: f64, t510: f64, t5113: f64, t574: f64, t650: f64, t652: f64, t671: f64, t88: f64, t9348: f64, t9416: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t45510 = piecewise3(t110, 0.0_f64, t45509);
    let t45545 = (2.0_f64 * t1268 * t45510 + 24.0_f64 * t12734 * t2363 + 8.0_f64 * t2314 * t9416 + 12.0_f64 * t2363 * t9348 + 6.0_f64 * t39231 * t88 + 8.0_f64 * t39235 * t671 + 8.0_f64 * t5113 * t9416 + t39223 + 12.0_f64 * t45408) * t574 - 12.0_f64 * t9348 * t2364 - 8.0_f64 * t2314 * t11972 - 24.0_f64 * t4034 * t12507 - 8.0_f64 * t4034 * t11972 - 8.0_f64 * t652 * t1266 * t9416 - 24.0_f64 * t9348 * t2323 - 8.0_f64 * t652 * t11968 * t671 - 24.0_f64 * t2314 * t12504 - 12.0_f64 * t652 * t3652 * t2363 - 6.0_f64 * t2312 * t3652 - 12.0_f64 * t45408 * t510 - 4.0_f64 * t650 * t11968 - 2.0_f64 * t652 * t510 * t45510;
    (t45510, t45545)
}
