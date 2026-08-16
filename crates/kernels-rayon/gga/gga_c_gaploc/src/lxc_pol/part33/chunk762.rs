//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 762/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk762(t2580: f64, t7245: f64, t1841: f64, t1897: f64, t2504: f64, t2508: f64, t2509: f64, t2577: f64, t5227: f64, t5288: f64, t5293: f64, t5524: f64, t7129: f64, t7137: f64, t7204: f64, t7207: f64, t7212: f64, t7215: f64, t7223: f64, t7228: f64, t7233: f64, t7236: f64, t7239: f64, t7242: f64) -> f64 {
    let t7246 = t2580 * t7245;
    let t7249 = -0.20508069947045931424e-1_f64 * t5293 * t2504 + 0.20508069947045931424e-1_f64 * t7137 * t2509 - 0.15381052460284448567e-1_f64 * t5288 * t2504 - 0.23071578690426672851e-1_f64 * t2508 * t7204 - 0.85450291446024714264e-3_f64 * t7207 + 0.32043859292259267849e-3_f64 * t7212 + 0.64087718584518535698e-3_f64 * t7215 - 0.8545029144602471425e-3_f64 * t5524 * t2577 + 0.17090058289204942853e-2_f64 * t5227 * t2577 + 0.17090058289204942853e-2_f64 * t1841 * t7223 - 0.46143157380853345701e-1_f64 * t2508 * t7228 + 0.15381052460284448567e-1_f64 * t7129 * t2509 - 0.15381052460284448567e-1_f64 * t1897 * t7233 - 0.76905262301422242837e-2_f64 * t1897 * t7236 + 0.76905262301422242837e-2_f64 * t2508 * t7239 + 0.15381052460284448567e-1_f64 * t2508 * t7242 + 0.15381052460284448567e-1_f64 * t2508 * t7246;
    t7249
}
