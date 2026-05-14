//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 930/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk930<F: Float>(t18325: F, t2012: F, t2020: F, t5014: F, t5507: F, t10879: F, t2637: F, t2013: F, t5005: F, t964: F, t2630: F, t5477: F, t2634: F, t17182: F, t7633: F, t4998: F, t7628: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18326 = t2012 * t18325;
    let t18327 = t5014 * t2020;
    let t18338 = t5014 * t5507;
    let t18355 = t10879 * t2637;
    let t18356 = t2013 * t18355;
    let t18372 = t964 * t5005;
    let t18406 = t2630 * t5477;
    let t18408 = t2634 * t5477;
    let t18421 = t17182 * t7633;
    let t18423 = 0.35981577432354634426e-1 * t2013 * t18421;
    let t18442 = t4998 * t7628;
    (t18326, t18327, t18338, t18356, t18372, t18406, t18408, t18423, t18442)
}
