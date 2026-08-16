//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1082/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1082(t1839: f64, t463: f64, t5873: f64, t7822: f64, t7493: f64, t8480: f64, t8648: f64, t6332: f64, t8511: f64, t1755: f64, t30540: f64, t7433: f64, t9674: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38685 = t1839 * t463;
    let t38701 = t7822 * t5873;
    let t38704 = t7493 * t8480 * t8648;
    let t38706 = t8511 * t6332;
    let t38709 = t30540 * t1755;
    let t38711 = t7433 * t9674;
    (t38685, t38701, t38704, t38706, t38709, t38711)
}
