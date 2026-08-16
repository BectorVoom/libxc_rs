//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1052/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1052(t10073: f64, t26261: f64, t1426: f64, t2098: f64, t786: f64, t136: f64, t2102: f64, t2457: f64, t25944: f64, t2470: f64, t7514: f64, t7284: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26263 = 0.24093411633903331839e-3_f64 * t10073 * t26261;
    let t26264 = t2098 * t1426;
    let t26265 = t786 * t26264;
    let t26276 = t2102 * t136;
    let t26277 = t26276 * t2457;
    let t26279 = 0.17135234354032049604e-2_f64 * t25944 * t26277;
    let t26292 = t7514 * t2470;
    let t26294 = 0.96373646535613327357e-2_f64 * t7284 * t26292;
    (t26263, t26264, t26265, t26276, t26277, t26279, t26292, t26294)
}
