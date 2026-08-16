//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1186/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1186(t5: f64, t84202: f64, t84231: f64, t84258: f64, t84287: f64, t112: f64, t1268: f64, t12734: f64, t12739: f64, t2039: f64, t2314: f64, t2363: f64, t23917: f64, t23938: f64, t26977: f64, t39235: f64, t45602: f64, t45637: f64, t45814: f64, t5113: f64, t671: f64, t7042: f64, t7056: f64, t84044: f64, t84097: f64, t84149: f64, t9348: f64, t9416: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t84290 = piecewise3(t8, 0.0_f64, t84202 + t84231 + t84258 + t84287);
    let t84291 = t84290 * t112;
    let t84298 = 2.0_f64 * t1268 * t84044 + 12.0_f64 * t12734 * t7056 + 6.0_f64 * t12739 * t7056 + 2.0_f64 * t2039 * t39235 + 6.0_f64 * t2039 * t45602 + 6.0_f64 * t2039 * t45637 + 2.0_f64 * t2039 * t45814 + 6.0_f64 * t2314 * t23917 + 6.0_f64 * t2363 * t23938 + 6.0_f64 * t2363 * t26977 + 6.0_f64 * t23917 * t5113 + 6.0_f64 * t671 * t84097 + 2.0_f64 * t7042 * t9416 + 6.0_f64 * t7056 * t9348 + 6.0_f64 * t84149 + t84291;
    (t84291, t84298)
}
