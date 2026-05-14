//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 518/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk518<F: Float>(t1008: F, t1086: F, t1092: F, t1098: F, t1005: F, t1103: F, t1108: F, t1113: F, t952: F, t957: F, t935: F, t940: F, t950: F, t151: F, t947: F, t377: F, t941: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3306 = t1008 * t1086;
    let t3308 = t1008 * t1092;
    let t3310 = t1008 * t1098;
    let t3312 = t1005 * t1103;
    let t3314 = t1005 * t1108;
    let t3316 = t1005 * t1113;
    let t3324 = t952 * t957;
    let t3326 = t935 * t957;
    let t3328 = t940 * t950;
    let t3329 = t151 * t3328;
    let t3330 = t3329 * t947;
    let t3343 = t377 * t941;
    (t3306, t3308, t3310, t3312, t3314, t3316, t3324, t3326, t3330, t3343)
}
