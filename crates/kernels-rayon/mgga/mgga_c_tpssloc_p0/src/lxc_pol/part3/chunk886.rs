//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 886/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk886(t731: f64, t9751: f64, t746: f64, t9490: f64, t172: f64, t9489: f64, t9493: f64, t9720: f64, t2512: f64, t9711: f64, t9689: f64, t9692: f64, t9695: f64, t9698: f64, t9702: f64, t9704: f64, t9706: f64, t9709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9752 = t9751 * t731;
    let t9755 = t9490 * t746;
    let t9758 = t172 * t9489;
    let t9759 = t9490 * t9493;
    let t9762 = t172 * t9720;
    let t9763 = t9490 * t2512;
    let t9766 = t9711 * t746;
    let t9777 = -0.25319e1_f64 * t9689 + 0.16879333333333333333e1_f64 * t9692 - 0.19692555555555555555e1_f64 * t9695 - 0.93011851851851851854e0_f64 * t9698 + 0.13651666666666666667e0_f64 * t9702 - 0.27303333333333333333e0_f64 * t9704 - 0.3185388888888888889e0_f64 * t9706 - 0.36514074074074074075e0_f64 * t9709;
    (t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9777)
}
