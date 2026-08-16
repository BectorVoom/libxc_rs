//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 721/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk721(t2039: f64, t88: f64, t1390: f64, t2094: f64, t2229: f64, t3: f64, t2239: f64, t601: f64, t83: f64, t84: f64, t85: f64, t24: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9012 = t88 * t2039;
    let t9016 = t2094 * t1390;
    let t9222 = t2229 * t3;
    let t9223 = 1.0_f64 / t9222;
    let t9231 = t601 * t2239;
    let t9238 = 1.0_f64 / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    (t9012, t9016, t9222, t9223, t9231, t9238, t9239)
}
