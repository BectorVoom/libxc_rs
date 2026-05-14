//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1080/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1080<F: Float>(t21503: F, t22090: F, t2508: F, t8604: F, t10752: F, t5293: F, t7667: F, t8637: F, t24660: F, t7659: F, t10731: F, t7129: F, t32356: F, t688: F, t779: F, t10682: F, t2060: F) -> (F, F, F, F, F, F, F) {
    let t32456 = 0.1845726295234133828e0 * t2508 * t22090 * t8604 * t21503;
    let t32458 = 0.6152420984113779427e-1 * t5293 * t10752;
    let t32461 = 0.10766736722199113997e0 * t2508 * t8637 * t7667;
    let t32464 = 0.1845726295234133828e0 * t2508 * t24660 * t7659;
    let t32466 = 0.18457262952341338281e0 * t7129 * t10731;
    let t32471 = 0.15381052460284448567e-1 * t2508 * t779 * t32356 * t688;
    let t32474 = 0.76905262301422242837e-2 * t2508 * t2060 * t10682;
    (t32456, t32458, t32461, t32464, t32466, t32471, t32474)
}
