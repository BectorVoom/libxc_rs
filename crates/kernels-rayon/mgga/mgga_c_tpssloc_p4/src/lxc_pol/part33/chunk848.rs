//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 848/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk848(t521: f64, t9861: f64, t17: f64, t1294: f64, t9494: f64, t1995: f64, t68: f64, t215: f64, t535: f64, t9569: f64, t1314: f64, t2559: f64) -> (f64, f64, f64, f64, f64) {
    let t12132 = t521 * t9861;
    let t12133 = t17 * t12132;
    let t12141 = 0.10254018858216406658e4_f64 * t1294 * t9494;
    let t12155 = t68 * t1995;
    let t12188 = 0.28086419753086419752e-1_f64 * t9569 * t535 * t215;
    let t12189 = t2559 * t1314;
    (t12133, t12141, t12155, t12188, t12189)
}
