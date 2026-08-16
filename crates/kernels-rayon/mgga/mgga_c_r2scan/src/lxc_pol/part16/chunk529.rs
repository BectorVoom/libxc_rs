//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 529/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk529(t106: f64, t2850: f64, t797: f64, t97: f64, t986: f64, t2266: f64, t481: f64, t104: f64) -> (f64, f64, f64, f64) {
    let t2853 = t97 * t106 * t2850 * t797;
    let t2854 = t986 * t797;
    let t2856 = t2266 * t2854 * t481;
    let t2857 = 3.0_f64 * t2856;
    let t2858 = t97 * t104;
    (t2853, t2854, t2857, t2858)
}
