//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 446/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk446(t1484: f64, t210: f64, t214: f64, t785: f64, t787: f64, t797: f64, t252: f64, t119: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1489 = t210 * t214 * t1484;
    let t1492 = -t785 - 0.16666666666666666666e-2_f64 * t787 * t1489 - t797;
    let t1493 = t1492 * t252;
    let t1495 = t119 * t1484;
    let t1496 = t210 * t1495;
    let t1499 = t1492 * t225;
    (t1489, t1492, t1493, t1495, t1496, t1499)
}
