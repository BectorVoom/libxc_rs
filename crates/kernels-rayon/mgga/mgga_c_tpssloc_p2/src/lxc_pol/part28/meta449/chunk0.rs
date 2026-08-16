//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1640/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1640(t5: f64, t24006: f64, t112: f64, t1268: f64, t12734: f64, t12739: f64, t2039: f64, t2314: f64, t2363: f64, t23917: f64, t23938: f64, t23941: f64, t5113: f64, t671: f64, t7042: f64, t7056: f64, t9348: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t24007 = piecewise3(t8, 0.0_f64, t24006);
    let t24008 = t24007 * t112;
    let t24026 = 2.0_f64 * t1268 * t23917 + 4.0_f64 * t12734 * t2039 + 2.0_f64 * t12739 * t2039 + 2.0_f64 * t2039 * t9348 + 4.0_f64 * t2314 * t7056 + 2.0_f64 * t2363 * t7042 + 4.0_f64 * t23938 * t671 + 4.0_f64 * t5113 * t7056 + 2.0_f64 * t23941 + t24008;
    (t24007, t24008, t24026)
}
