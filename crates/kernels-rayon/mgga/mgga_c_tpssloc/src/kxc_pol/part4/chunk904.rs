//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 904/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk904(t3247: f64, t405: f64, t974: f64, t457: f64, t63: f64, t461: f64, t221: f64, t456: f64, t1186: f64, t698: f64, t1174: f64, t1184: f64, t4899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11545 = 1.0_f64 / t405 / t3247;
    let t11546 = t974 * t11545;
    let t11552 = t63 * t457;
    let t11553 = t11552 * t461;
    let t11554 = t221 * t11553;
    let t11556 = 0.3086419753086419753e-3_f64 * t456 * t11554;
    let t11557 = t698 * t1186;
    let t11558 = t1174 * t11557;
    let t11569 = t4899 * t1184;
    (t11545, t11546, t11552, t11556, t11558, t11569)
}
