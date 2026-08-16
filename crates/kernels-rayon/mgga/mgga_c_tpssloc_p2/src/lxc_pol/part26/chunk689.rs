//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 689/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk689(t1338: f64, t68: f64, t544: f64, t2235: f64, t33: f64, t645: f64, t79: f64, t72: f64, t605: f64, t608: f64, t625: f64, t641: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5343 = t68 * t1338;
    let t5344 = t544 * t5343;
    let t6486 = t2235 * t33;
    let t6491 = t79 * t645;
    let t6492 = t72 * t6491;
    let t6495 = t605 * t608;
    let t6503 = 8.0_f64 / 3.0_f64 * t625;
    let t6509 = t71 * t641;
    (t5344, t6486, t6492, t6495, t6503, t6509)
}
