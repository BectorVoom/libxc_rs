//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1654/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1654(t12178: f64, t1380: f64, t3856: f64, t3901: f64, t215: f64, t535: f64, t9569: f64, t1314: f64, t2559: f64) -> (f64, f64, f64, f64) {
    let t12179 = t1380 * t12178;
    let t12181 = t3901 * t3856;
    let t12188 = 0.28086419753086419752e-1_f64 * t9569 * t535 * t215;
    let t12189 = t2559 * t1314;
    (t12179, t12181, t12188, t12189)
}
