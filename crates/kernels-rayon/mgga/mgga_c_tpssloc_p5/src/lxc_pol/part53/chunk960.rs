//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 960/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk960(t225: f64, t32164: f64, t115390: f64, t115432: f64, t115434: f64, t113981: f64, t114025: f64, t114027: f64, t114038: f64, t1338: f64, t32147: f64, t32168: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t117173 = t32164 * t225;
    let t117193 = 0.3289868133696452873e-1_f64 * t115390;
    let t117209 = 0.10417915756705434098e0_f64 * t115432;
    let t117210 = 0.25587863262083522346e0_f64 * t115434;
    let t117217 = 0.26915170729426927235e-3_f64 * t113981;
    let t117231 = 0.84334201618871038669e-2_f64 * t114025;
    let t117232 = 0.18086994730174895102e0_f64 * t114027;
    let t117235 = 119.0_f64 / 1728.0_f64 * t114038;
    let t117246 = t1338 * t32147;
    let t117275 = t32168 * t225;
    (t117173, t117193, t117209, t117210, t117217, t117231, t117232, t117235, t117246, t117275)
}
