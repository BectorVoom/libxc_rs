//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 579/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk579<F: Float>(t8469: F, t935: F, t2580: F, t2508: F, t2530: F, t2958: F, t10677: F, t701: F, t1901: F, t7659: F, t9014: F, t3444: F, t731: F) -> (F, F, F, F, F, F, F, F) {
    let t10713 = t8469 * t935;
    let t10714 = t2580 * t10713;
    let t10716 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t10714;
    let t10717 = t2958 * t2530;
    let t10718 = t2580 * t10717;
    let t10720 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t10718;
    let t10721 = t10677 * t701;
    let t10722 = t1901 * t10721;
    let t10731 = t9014 * t7659;
    let t10733 = F::cast_from(0.92286314761706691403e-1_f64) * t2508 * t10731;
    let t10734 = t731 * t3444;
    (t10713, t10716, t10717, t10720, t10721, t10722, t10733, t10734)
}
