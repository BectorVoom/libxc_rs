//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1104/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1104<F: Float>(t21503: F, t22090: F, t2508: F, t8604: F, t10752: F, t5293: F, t7667: F, t8637: F, t24660: F, t7659: F, t10731: F, t7129: F, t32429: F, t32431: F, t32434: F, t32439: F, t32441: F, t32443: F, t32446: F, t32448: F, t32452: F) -> (F,) {
    let t32456 = 0.1845726295234133828e0 * t2508 * t22090 * t8604 * t21503;
    let t32458 = 0.6152420984113779427e-1 * t5293 * t10752;
    let t32461 = 0.10766736722199113997e0 * t2508 * t8637 * t7667;
    let t32464 = 0.1845726295234133828e0 * t2508 * t24660 * t7659;
    let t32466 = 0.18457262952341338281e0 * t7129 * t10731;
    let t32467 = -t32429 + t32431 + t32434 - t32439 - t32441 + t32443 - t32446 - t32448 - t32452 + t32456 + t32458 - t32461 + t32464 + t32466;
    (t32467,)
}
