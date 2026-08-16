//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 595/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk595(t158: f64, t3246: f64, t1255: f64, t2428: f64, t951: f64, t1227: f64, t410: f64, t2363: f64, t2970: f64, t3187: f64, t133: f64, t3199: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3247 = t3246 * t158;
    let t3254 = t2428 * t1255;
    let t3255 = t3254 * t951;
    let t3258 = t410 * t1227;
    let t3259 = t2363 * t3258;
    let t3260 = t2970 * t3187;
    let t3265 = t3199 * t133;
    (t3247, t3254, t3255, t3258, t3259, t3260, t3265)
}
