//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1964/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1964(t109: f64, t86586: f64, t86588: f64, t86590: f64, t81440: f64, t81443: f64, t81445: f64, t84036: f64, t86593: f64, t86596: f64, t86599: f64, t86601: f64, t1268: f64, t12725: f64, t12734: f64, t12739: f64, t19456: f64, t2039: f64, t2314: f64, t23917: f64, t26114: f64, t26117: f64, t27170: f64, t5113: f64, t55934: f64, t7056: f64, t7676: f64, t7801: f64, t90370: f64, t90375: f64, t9348: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t92121 = 22.0_f64 / 9.0_f64 * t86586;
    let t92122 = 8.0_f64 / 3.0_f64 * t86588;
    let t92123 = 4.0_f64 / 3.0_f64 * t86590;
    let t92127 = -t84036 - 44.0_f64 / 9.0_f64 * t81440 - 4.0_f64 / 3.0_f64 * t81443 + 2.0_f64 / 3.0_f64 * t81445 - t92121 - t92122 + t92123 - 3.0_f64 / 2.0_f64 * t86593 + t86596 + t86599 / 2.0_f64 - t86601 / 4.0_f64;
    let t92128 = piecewise3(t110, 0.0_f64, t92127);
    let t92139 = 2.0_f64 * t1268 * t92128 + 4.0_f64 * t12725 * t7056 + 4.0_f64 * t12734 * t7801 + 2.0_f64 * t12739 * t7801 + 4.0_f64 * t19456 * t7056 + 4.0_f64 * t2039 * t55934 + 4.0_f64 * t2039 * t90370 + 2.0_f64 * t2039 * t90375 + 4.0_f64 * t2314 * t27170 + 2.0_f64 * t23917 * t7676 + 4.0_f64 * t26114 * t7056 + 4.0_f64 * t26117 * t7056 + 4.0_f64 * t27170 * t5113 + 2.0_f64 * t7801 * t9348;
    (t92128, t92139)
}
