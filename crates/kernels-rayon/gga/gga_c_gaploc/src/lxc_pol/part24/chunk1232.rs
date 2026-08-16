//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1232/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1232(t21503: f64, t2508: f64, t3009: f64, t7226: f64, t22090: f64, t8604: f64, t10752: f64, t5293: f64, t7667: f64, t8637: f64, t24660: f64, t7659: f64) -> (f64, f64, f64, f64, f64) {
    let t32452 = 0.46143157380853345701e0_f64 * t2508 * t7226 * t3009 * t21503;
    let t32456 = 0.1845726295234133828e0_f64 * t2508 * t22090 * t8604 * t21503;
    let t32458 = 0.6152420984113779427e-1_f64 * t5293 * t10752;
    let t32461 = 0.10766736722199113997e0_f64 * t2508 * t8637 * t7667;
    let t32464 = 0.1845726295234133828e0_f64 * t2508 * t24660 * t7659;
    (t32452, t32456, t32458, t32461, t32464)
}
