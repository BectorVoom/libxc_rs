//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1228/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1228<F: Float>(t3437: F, t7211: F, t10749: F, t2549: F, t10640: F, t7129: F, t10688: F, t10776: F, t10779: F, t10782: F, t10790: F, t1897: F, t1901: F, t2095: F, t2508: F, t2580: F, t32387: F, t32394: F, t32398: F, t32400: F, t32408: F, t32411: F, t3451: F, t5269: F, t5293: F, t5397: F, t5524: F, t7137: F) -> F {
    let t32412 = t7211 * t3437;
    let t32413 = F::new(0.32043859292259267849e-3) * t32412;
    let t32414 = t2549 * t10749;
    let t32415 = F::new(0.64087718584518535698e-3) * t32414;
    let t32417 = F::new(0.92286314761706691402e-1) * t7129 * t10640;
    let t32424 = -F::new(0.15381052460284448567e-1) * t5269 * t1901 * t32387 - F::new(0.20508069947045931424e-1) * t5293 * t10776 - t32394 - F::new(0.61524209841137794271e-1) * t7137 * t10790 + t32398 - t32400 + F::new(0.20508069947045931424e-1) * t7137 * t10779 - F::new(0.53833683610995569986e-1) * t2508 * t2095 * t3451 - t32408 + t32411 + t32413 + t32415 - t32417 - F::new(0.30762104920568897134e-1) * t1897 * t2580 * t10782 * t5397 + F::new(0.8545029144602471425e-3) * t5524 * t10688;
    t32424
}
