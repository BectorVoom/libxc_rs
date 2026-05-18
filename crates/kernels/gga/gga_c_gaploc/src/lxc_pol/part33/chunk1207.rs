//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1207/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1207<F: Float>(t21503: F, t2508: F, t3009: F, t7226: F, t22090: F, t8604: F, t10752: F, t5293: F, t7667: F, t8637: F, t24660: F, t7659: F) -> (F, F, F, F, F) {
    let t32452 = F::new(0.46143157380853345701e0) * t2508 * t7226 * t3009 * t21503;
    let t32456 = F::new(0.1845726295234133828e0) * t2508 * t22090 * t8604 * t21503;
    let t32458 = F::new(0.6152420984113779427e-1) * t5293 * t10752;
    let t32461 = F::new(0.10766736722199113997e0) * t2508 * t8637 * t7667;
    let t32464 = F::new(0.1845726295234133828e0) * t2508 * t24660 * t7659;
    (t32452, t32456, t32458, t32461, t32464)
}
