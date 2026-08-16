//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2030/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2030(t831: f64, t87261: f64, t4191: f64, t81749: f64, t4240: f64, t23069: f64, t4159: f64, t23062: f64, t25106: f64, t13176: f64, t6613: f64, t23133: f64, t4257: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87262 = t87261 * t831;
    let t87263 = 7.0_f64 / 1152.0_f64 * t87262;
    let t87270 = t81749 * t4191;
    let t87271 = 7.0_f64 / 288.0_f64 * t87270;
    let t87272 = t81749 * t4240;
    let t87273 = 7.0_f64 / 1152.0_f64 * t87272;
    let t87291 = t23069 * t4159;
    let t87292 = 7.0_f64 / 72.0_f64 * t87291;
    let t87293 = t23062 * t25106;
    let t87295 = t13176 * t6613;
    let t87300 = t23133 * t4257;
    (t87263, t87271, t87273, t87292, t87293, t87295, t87300)
}
