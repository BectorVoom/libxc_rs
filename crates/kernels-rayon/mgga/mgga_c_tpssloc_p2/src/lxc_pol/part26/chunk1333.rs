//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1333/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1333(t1873: f64, t45637: f64, t12734: f64, t6534: f64, t39235: f64, t45602: f64, t9348: f64, t1268: f64, t81455: f64, t22479: f64, t2314: f64, t45814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t83946 = 6.0_f64 * t45637 * t1873;
    let t83948 = 12.0_f64 * t12734 * t6534;
    let t83952 = 2.0_f64 * t39235 * t1873;
    let t83956 = 6.0_f64 * t45602 * t1873;
    let t83958 = 6.0_f64 * t9348 * t6534;
    let t83960 = 2.0_f64 * t1268 * t81455;
    let t83962 = 6.0_f64 * t2314 * t22479;
    let t83964 = 2.0_f64 * t45814 * t1873;
    (t83946, t83948, t83952, t83956, t83958, t83960, t83962, t83964)
}
