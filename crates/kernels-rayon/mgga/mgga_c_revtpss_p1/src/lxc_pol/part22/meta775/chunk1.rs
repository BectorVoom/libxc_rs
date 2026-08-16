//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2864/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2864(t1122: f64, t1261: f64, t247: f64, t44701: f64, t11262: f64, t3711: f64, t3713: f64, t12657: f64, t1284: f64, t3624: f64, t221: f64, t461: f64, t462: f64, t624: f64) -> (f64, f64, f64, f64) {
    let t44704 = t1261 * t247 * t44701 * t1122;
    let t44751 = t3711 * t11262 * t3713;
    let t44769 = t12657 * t1284 * t3624;
    let t44797 = 5.0_f64 / 486.0_f64 * t461 * t221 * t624 * t462;
    (t44704, t44751, t44769, t44797)
}
