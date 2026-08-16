//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1058/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1058(t491: f64, t6150: f64, t1720: f64, t1751: f64, t1730: f64, t1743: f64, t1417: f64, t47: f64, t480: f64, t479: f64, t471: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6151 = t6150 * t491;
    let t6153 = t1720 * t1751;
    let t6158 = t1730 * t1743;
    let t6163 = 1.0_f64 / t47 / t480 / t1417;
    let t6164 = t479 * t6163;
    let t6165 = t471 * t6164;
    let t6168 = t6150 * t225;
    (t6151, t6153, t6158, t6163, t6164, t6165, t6168)
}
