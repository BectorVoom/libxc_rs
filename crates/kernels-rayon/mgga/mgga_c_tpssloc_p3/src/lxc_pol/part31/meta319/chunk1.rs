//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1211/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1211(t12129: f64, t17: f64, t521: f64, t9861: f64, t3826: f64, t592: f64, t1285: f64, t2225: f64, t2371: f64, t3691: f64, t1294: f64, t9494: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12130 = t17 * t12129;
    let t12132 = t521 * t9861;
    let t12133 = t17 * t12132;
    let t12134 = t592 * t3826;
    let t12136 = t2225 * t1285;
    let t12138 = t3691 * t2371;
    let t12141 = 0.10254018858216406658e4_f64 * t1294 * t9494;
    (t12130, t12133, t12134, t12136, t12138, t12141)
}
