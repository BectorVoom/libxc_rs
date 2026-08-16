//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 870/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk870(t138: f64, t2409: f64, t125: f64, t2412: f64, t701: f64, t2414: f64) -> (f64, f64) {
    let t9452 = 1.0_f64 / t2409 / t138;
    let t9453 = t125 * t9452;
    let t9454 = t2412 * t701;
    let t9455 = t9454 * t2414;
    let t9457 = 0.96491876992155210402e2_f64 * t9453 * t9455;
    (t9454, t9457)
}
