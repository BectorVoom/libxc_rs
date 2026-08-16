//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 754/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk754(t1850: f64, t1908: f64, t1935: f64, t1939: f64, t2533: f64, t2583: f64, t2587: f64, t270: f64, t5269: f64, t650: f64, t681: f64, t7066: f64, t7070: f64, t7115: f64, t7125: f64, t7129: f64, t7133: f64, t7137: f64, t938: f64, t949: f64) -> f64 {
    let t7140 = -0.64087718584518535698e-3_f64 * t7066 - 0.17090058289204942853e-2_f64 * t1850 * t7070 + 0.34180116578409885707e-2_f64 * t1908 * t938 - 0.20508069947045931424e-1_f64 * t1939 * t949 - 0.20508069947045931424e-1_f64 * t650 * t2587 - 0.76905262301422242837e-2_f64 * t1935 * t949 - 0.15381052460284448567e-1_f64 * t681 * t2587 - 0.34180116578409885707e-2_f64 * t1908 * t949 + 0.15381052460284448567e-1_f64 * t681 * t2533 + 0.76905262301422242837e-2_f64 * t270 * t7115 + 0.20508069947045931424e-1_f64 * t1939 * t938 + 0.20508069947045931424e-1_f64 * t650 * t2533 + 0.76905262301422242837e-2_f64 * t1935 * t938 - 0.76905262301422242837e-2_f64 * t270 * t7125 + 0.30762104920568897134e-1_f64 * t7129 * t2583 - 0.15381052460284448567e-1_f64 * t5269 * t7133 + 0.41016139894091862847e-1_f64 * t7137 * t2583;
    t7140
}
