//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 82/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk82(t60: f64, t67: f64, t10: f64, t260: f64, t116: f64) -> (f64, f64, f64) {
    let t261 = t67 * t60;
    let t264 = 10.0_f64 / 9.0_f64 * t260 * t261 * t10;
    let t265 = t264 < -0.66725e-1_f64;
    let t267 = piecewise3(t265, 0.0_f64, 0.66725e-1_f64 + t264);
    let t268 = t267 * t116;
    (t261, t268, t264)
}
