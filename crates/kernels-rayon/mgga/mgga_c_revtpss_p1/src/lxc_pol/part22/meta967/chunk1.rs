//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3232/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3232(t50113: f64, t40150: f64, t14341: f64, t4311: f64, t18253: f64, t18268: f64, t198: f64, t2394: f64, t2430: f64, t262: f64, t39989: f64, t4541: f64, t50080: f64, t5966: f64, t61265: f64, t61269: f64, t61274: f64, t61283: f64, t61286: f64) -> (f64, f64, f64, f64) {
    let t61287 = 8.0_f64 * t50113;
    let t61288 = 2.0_f64 * t40150;
    let t61289 = t4311 * t14341;
    let t61290 = 16.0_f64 * t61289;
    let t61291 = 6.0_f64 * t198 * t2430 * t262 * t5966 - 6.0_f64 * t18268 * t2394 * t4541 + 24.0_f64 * t18253 * t50080 - t39989 + t61265 + t61269 + t61274 + t61283 + t61286 + t61287 + t61288 + t61290;
    (t61287, t61288, t61290, t61291)
}
