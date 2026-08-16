//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 877/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk877(t72: f64, t7531: f64, t686: f64, t7284: f64, t7289: f64, t136: f64, t2102: f64, t2457: f64, t25944: f64, t25950: f64, t7515: f64, t213: f64, t7506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26270 = t7531 * t72;
    let t26271 = t26270 * t686;
    let t26272 = t7284 * t26271;
    let t26274 = t7289 * t26271;
    let t26276 = t2102 * t136;
    let t26277 = t26276 * t2457;
    let t26279 = 0.17135234354032049604e-2_f64 * t25944 * t26277;
    let t26280 = t25950 * t7515;
    let t26282 = t213 * t7506;
    (t26272, t26274, t26277, t26279, t26280, t26282)
}
