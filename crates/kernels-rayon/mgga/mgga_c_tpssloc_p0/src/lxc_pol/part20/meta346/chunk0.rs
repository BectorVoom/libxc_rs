//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1648/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1648(t3824: f64, t588: f64, t1287: f64, t2225: f64, t12083: f64, t184: f64, t17: f64, t3681: f64, t750: f64, t1284: f64, t2516: f64, t521: f64, t9861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12120 = t588 * t3824;
    let t12121 = 12.0_f64 * t12120;
    let t12123 = 60.0_f64 * t2225 * t1287;
    let t12124 = t12083 * t184;
    let t12125 = t17 * t12124;
    let t12126 = t3681 * t750;
    let t12127 = t17 * t12126;
    let t12128 = 3.0_f64 * t12127;
    let t12129 = t1284 * t2516;
    let t12130 = t17 * t12129;
    let t12131 = 3.0_f64 * t12130;
    let t12132 = t521 * t9861;
    (t12120, t12121, t12123, t12124, t12125, t12126, t12127, t12128, t12129, t12130, t12131, t12132)
}
