//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2352/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2352(t20896: f64, t2697: f64, t13360: f64, t5624: f64, t1516: f64, t58844: f64, t5628: f64, t67441: f64, t842: f64, t59263: f64, t59276: f64, t59279: f64, t59282: f64, t59288: f64, t59298: f64, t59308: f64, t59310: f64, t59322: f64, t849: f64) -> f64 {
    let t68195 = t2697 * t20896;
    let t68197 = t13360 * t5624;
    let t68199 = t58844 * t1516;
    let t68201 = t13360 * t5628;
    let t68203 = t67441 * t842;
    let t68207 = -119.0_f64 / 1152.0_f64 * t59263 - 119.0_f64 / 4608.0_f64 * t59276 + 35.0_f64 / 64.0_f64 * t59279 + 7.0_f64 / 1536.0_f64 * t59282 + 119.0_f64 / 4608.0_f64 * t59288 - 7.0_f64 / 16.0_f64 * t59298 - 7.0_f64 / 1536.0_f64 * t59308 - 7.0_f64 / 8.0_f64 * t59310 + 35.0_f64 / 192.0_f64 * t68195 - 35.0_f64 / 384.0_f64 * t68197 + 7.0_f64 / 384.0_f64 * t68199 + 7.0_f64 / 384.0_f64 * t68201 - t68203 * t849 / 768.0_f64 + 35.0_f64 / 192.0_f64 * t59322;
    t68207
}
