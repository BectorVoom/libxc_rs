//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 512/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk512<F: Float>(t8469: F, t935: F, t2580: F, t2508: F, t2530: F, t2958: F, t7659: F, t9014: F, t3444: F, t731: F, t7068: F, t1897: F, t2549: F, t8528: F, t883: F, t2562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10713 = t8469 * t935;
    let t10714 = t2580 * t10713;
    let t10716 = 0.15381052460284448567e-1 * t2508 * t10714;
    let t10717 = t2958 * t2530;
    let t10718 = t2580 * t10717;
    let t10720 = 0.15381052460284448567e-1 * t2508 * t10718;
    let t10731 = t9014 * t7659;
    let t10733 = 0.92286314761706691403e-1 * t2508 * t10731;
    let t10734 = t731 * t3444;
    let t10735 = 0.42725145723012357132e-3 * t10734;
    let t10736 = t2958 * t7068;
    let t10737 = t2580 * t10736;
    let t10739 = 0.15381052460284448567e-1 * t1897 * t10737;
    let t10740 = t2549 * t3444;
    let t10741 = 0.32043859292259267849e-3 * t10740;
    let t10742 = t883 * t8528;
    let t10743 = t2562 * t10742;
    (t10713, t10716, t10717, t10720, t10733, t10734, t10735, t10736, t10739, t10740, t10741, t10742, t10743)
}
