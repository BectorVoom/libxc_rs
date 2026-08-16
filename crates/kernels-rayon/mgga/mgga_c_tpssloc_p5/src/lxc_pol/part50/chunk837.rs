//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 837/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk837(t1873: f64, t88: f64, t2018: f64, t3701: f64, t192: f64, t533: f64, t1390: f64, t2229: f64, t3: f64, t2239: f64, t601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8601 = t88 * t1873;
    let t8643 = t3701 * t2018;
    let t8944 = t192 * t533;
    let t8945 = t2018 * t1390;
    let t9222 = t2229 * t3;
    let t9223 = 1.0_f64 / t9222;
    let t9231 = t601 * t2239;
    (t8601, t8643, t8944, t8945, t9222, t9223, t9231)
}
