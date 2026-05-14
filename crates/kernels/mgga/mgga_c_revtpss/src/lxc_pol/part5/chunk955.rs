//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 955/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk955<F: Float>(t11506: F, t315: F, t3013: F, t323: F, t2873: F, t910: F, t11132: F, t2942: F, t941: F, t2986: F, t960: F, t1034: F, t3154: F, t357: F, t1024: F, t3105: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11507 = t315 * t11506;
    let t11509 = 1.0 / t3013 / t323;
    let t11528 = t910 * t2873;
    let t11534 = 0.55403703703703703703e-1 * t11132;
    let t11548 = t941 * t2942;
    let t11554 = t960 * t2986;
    let t11560 = 0.28842592592592592592e-1 * t11132;
    let t11574 = 0.53272592592592592592e-1 * t11132;
    let t11626 = t1034 * t1034;
    let t11627 = 1.0 / t11626;
    let t11631 = t3154 * t357;
    let t11656 = t1024 * t3105;
    (t11507, t11509, t11528, t11534, t11548, t11554, t11560, t11574, t11627, t11631, t11656)
}
