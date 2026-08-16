//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1033/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1033(t9088: f64, t9621: f64, t9625: f64, t9628: f64, t9097: f64, t9107: f64, t9112: f64, t9114: f64, t9119: f64, t7908: f64, t7910: f64, t8223: f64, t8224: f64, t8293: f64, t8295: f64, t8297: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42559 = 0.1702583995731913576e-4_f64 * t9088;
    let t42560 = 0.23948483403727617128e0_f64 * t9621;
    let t42561 = 0.23948483403727617128e0_f64 * t9625;
    let t42562 = 0.23948483403727617128e0_f64 * t9628;
    let t42563 = 0.5107751987195740728e-4_f64 * t9097;
    let t42567 = 0.5107751987195740728e-4_f64 * t9107;
    let t42568 = 0.1702583995731913576e-4_f64 * t9112;
    let t42569 = 0.1702583995731913576e-4_f64 * t9114;
    let t42570 = 0.638468998399467591e-4_f64 * t9119;
    let t42571 = 0.79453919800822633544e-4_f64 * t7908 - 0.23836175940246790064e-3_f64 * t7910 - t8223 + t8224 + t8293 + t8295 + t8297 + t42567 + t42568 - t42569 - t42570;
    (t42559, t42560, t42561, t42562, t42563, t42571)
}
