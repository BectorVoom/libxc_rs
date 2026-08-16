//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1006/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1006(t12202: f64, t12204: f64, t118: f64, t3719: f64, t794: f64, t3739: f64, t3732: f64, t782: f64, t3736: f64, t1365: f64, t154: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12205 = t12202 * t12204;
    let t12208 = t118 * t794 * t3719;
    let t12209 = t3739 * t12208;
    let t12211 = t782 * t3732;
    let t12212 = t12211 * t3736;
    let t12214 = t154 * t1365;
    let t12215 = t205 * t12214;
    (t12205, t12209, t12211, t12212, t12214, t12215)
}
