//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1403/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1403(t3684: f64, t9888: f64, t9885: f64, t3824: f64, t588: f64, t1287: f64, t2225: f64, t3681: f64, t750: f64, t17: f64, t1284: f64, t2516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12116 = 0.48159733137676571078e0_f64 * t3684 * t9888;
    let t12118 = 0.16265371950452609763e-1_f64 * t3684 * t9885;
    let t12120 = t588 * t3824;
    let t12123 = 60.0_f64 * t2225 * t1287;
    let t12126 = t3681 * t750;
    let t12127 = t17 * t12126;
    let t12129 = t1284 * t2516;
    (t12116, t12118, t12120, t12123, t12127, t12129)
}
