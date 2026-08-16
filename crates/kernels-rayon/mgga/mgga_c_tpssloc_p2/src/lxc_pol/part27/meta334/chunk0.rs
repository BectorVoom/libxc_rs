//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1413/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1413(t1287: f64, t2225: f64, t3681: f64, t750: f64, t17: f64, t1284: f64, t2516: f64, t521: f64, t9861: f64, t3826: f64, t592: f64, t1285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12123 = 60.0_f64 * t2225 * t1287;
    let t12126 = t3681 * t750;
    let t12127 = t17 * t12126;
    let t12129 = t1284 * t2516;
    let t12130 = t17 * t12129;
    let t12132 = t521 * t9861;
    let t12133 = t17 * t12132;
    let t12134 = t592 * t3826;
    let t12136 = t2225 * t1285;
    (t12123, t12127, t12130, t12133, t12134, t12136)
}
