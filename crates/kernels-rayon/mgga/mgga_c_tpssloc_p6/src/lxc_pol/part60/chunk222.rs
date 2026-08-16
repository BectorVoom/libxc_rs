//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 222/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk222(t1222: f64, t485: f64, t372: f64, t483: f64, t479: f64, t471: f64, t404: f64, t415: f64, t61: f64, t225: f64, t492: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1224 = t485 * t1222 / 4608.0_f64;
    let t1225 = t483 * t372;
    let t1226 = t479 * t1225;
    let t1227 = t471 * t1226;
    let t1229 = 1.0_f64 / t415 / t404;
    let t1230 = t61 * t1229;
    let t1238 = t492 * t225;
    (t1224, t1225, t1226, t1227, t1229, t1230, t1238)
}
