//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 509/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk509<F: Float>(t1005: F, t1103: F, t1108: F, t1113: F, t940: F, t950: F, t151: F, t377: F, t941: F, t301: F, t864: F, t1089: F, t175: F, t1036: F, t182: F, t315: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3312 = t1005 * t1103;
    let t3314 = t1005 * t1108;
    let t3316 = t1005 * t1113;
    let t3328 = t940 * t950;
    let t3329 = t151 * t3328;
    let t3343 = t377 * t941;
    let t3355 = t864 * t301;
    let t3357 = t1089 * t175 * t3355;
    let t3358 = t1036 * t3357;
    let t3360 = t315 * t182;
    (t3312, t3314, t3316, t3329, t3343, t3355, t3357, t3358, t3360)
}
