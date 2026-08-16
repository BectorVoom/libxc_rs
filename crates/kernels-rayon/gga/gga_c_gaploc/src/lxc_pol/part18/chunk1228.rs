//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1228/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1228(t3437: f64, t7211: f64, t10749: f64, t2549: f64, t10640: f64, t7129: f64, t10688: f64, t10776: f64, t10779: f64, t10782: f64, t10790: f64, t1897: f64, t1901: f64, t2095: f64, t2508: f64, t2580: f64, t32387: f64, t32394: f64, t32398: f64, t32400: f64, t32408: f64, t32411: f64, t3451: f64, t5269: f64, t5293: f64, t5397: f64, t5524: f64, t7137: f64) -> f64 {
    let t32412 = t7211 * t3437;
    let t32413 = 0.32043859292259267849e-3_f64 * t32412;
    let t32414 = t2549 * t10749;
    let t32415 = 0.64087718584518535698e-3_f64 * t32414;
    let t32417 = 0.92286314761706691402e-1_f64 * t7129 * t10640;
    let t32424 = -0.15381052460284448567e-1_f64 * t5269 * t1901 * t32387 - 0.20508069947045931424e-1_f64 * t5293 * t10776 - t32394 - 0.61524209841137794271e-1_f64 * t7137 * t10790 + t32398 - t32400 + 0.20508069947045931424e-1_f64 * t7137 * t10779 - 0.53833683610995569986e-1_f64 * t2508 * t2095 * t3451 - t32408 + t32411 + t32413 + t32415 - t32417 - 0.30762104920568897134e-1_f64 * t1897 * t2580 * t10782 * t5397 + 0.8545029144602471425e-3_f64 * t5524 * t10688;
    t32424
}
