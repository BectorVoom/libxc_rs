//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1406/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1406(t3726: f64, t3745: f64, t1314: f64, t2566: f64, t3741: f64, t3732: f64, t792: f64, t118: f64, t3734: f64, t794: f64, t3719: f64, t3739: f64) -> (f64, f64, f64, f64, f64) {
    let t12197 = t3726 * t3745;
    let t12199 = t2566 * t1314;
    let t12200 = t12199 * t3741;
    let t12202 = t792 * t3732;
    let t12204 = t118 * t794 * t3734;
    let t12205 = t12202 * t12204;
    let t12208 = t118 * t794 * t3719;
    let t12209 = t3739 * t12208;
    (t12197, t12199, t12200, t12205, t12209)
}
