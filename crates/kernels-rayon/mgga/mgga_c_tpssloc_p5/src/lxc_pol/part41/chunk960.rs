//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 960/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk960(t3749: f64, t9577: f64, t1314: f64, t2566: f64, t3741: f64, t3732: f64, t792: f64, t782: f64, t1365: f64, t154: f64, t205: f64, t116: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12196 = 0.99999999999999999997e-2_f64 * t9577 * t3749;
    let t12199 = t2566 * t1314;
    let t12200 = t12199 * t3741;
    let t12202 = t792 * t3732;
    let t12211 = t782 * t3732;
    let t12214 = t154 * t1365;
    let t12215 = t205 * t12214;
    let t12225 = t547 * t116;
    (t12196, t12199, t12200, t12202, t12211, t12214, t12215, t12225)
}
