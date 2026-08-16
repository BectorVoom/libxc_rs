//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1092/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1092(t154: f64, t21: f64, t6896: f64, t6898: f64, t213: f64, t6924: f64, t9223: f64, t22715: f64, t547: f64, t22822: f64, t281: f64, t120: f64, t22816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80741 = t21 * t154;
    let t80742 = t80741 * t6896;
    let t80743 = t80742 * t6898;
    let t80766 = t9223 * t6924 * t213;
    let t80775 = t22715 * t547;
    let t80779 = t22822 * t6924 * t281;
    let t80782 = t22816 * t120;
    (t80741, t80742, t80743, t80766, t80775, t80779, t80782)
}
