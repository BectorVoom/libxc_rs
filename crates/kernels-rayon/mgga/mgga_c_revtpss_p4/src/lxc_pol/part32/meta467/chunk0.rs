//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1693/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1693(t26265: f64, t3917: f64, t25899: f64, t26231: f64, t72: f64, t7531: f64, t686: f64, t7284: f64, t7289: f64, t136: f64, t2102: f64, t2457: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26266 = t26265 * t3917;
    let t26268 = t25899 * t26231;
    let t26270 = t7531 * t72;
    let t26271 = t26270 * t686;
    let t26272 = t7284 * t26271;
    let t26274 = t7289 * t26271;
    let t26276 = t2102 * t136;
    let t26277 = t26276 * t2457;
    (t26266, t26268, t26270, t26271, t26272, t26274, t26276, t26277)
}
