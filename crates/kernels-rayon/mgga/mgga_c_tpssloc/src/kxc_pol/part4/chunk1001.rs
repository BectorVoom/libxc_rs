//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1001/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1001(t16710: f64, t758: f64, t3966: f64, t4195: f64, t4194: f64, t184: f64, t5392: f64, t607: f64, t12939: f64, t13121: f64, t16699: f64, t16700: f64, t16703: f64, t16705: f64, t16707: f64, t16708: f64, t16709: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64) -> (f64, f64, f64, f64) {
    let t16711 = t16710 * t758;
    let t16712 = 0.18311447306006545054e-3_f64 * t16711;
    let t16713 = t4195 * t3966;
    let t16715 = 24.0_f64 * t4194 * t16713;
    let t16716 = t184 * t5392;
    let t16717 = t16716 * t607;
    let t16719 = 24.0_f64 * t12939 * t16717;
    let t16720 = -t9894 + t16699 + t9907 - t16700 + t16703 + t9853 + t16705 + t16707 - t13121 - t9921 - t16708 + t16709 - t16712 + t16715 + t16719 + t9859;
    (t16712, t16715, t16719, t16720)
}
