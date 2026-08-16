//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1329/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1329(t3482: f64, t619: f64, t77: f64, t1313: f64, t2049: f64, t10408: f64, t76: f64, t10289: f64, t582: f64, t1993: f64, t3418: f64, t1982: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65321 = t77 * t3482 * t619;
    let t65325 = t77 * t1313 * t2049;
    let t65396 = t76 * t10408;
    let t65400 = t10289 * t582;
    let t65403 = t3418 * t1993;
    let t65410 = t77 * t1313 * t1982;
    (t65321, t65325, t65396, t65400, t65403, t65410)
}
