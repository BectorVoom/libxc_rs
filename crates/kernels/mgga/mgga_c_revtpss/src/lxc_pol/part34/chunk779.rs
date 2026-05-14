//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 779/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk779<F: Float>(t11449: F, t302: F, t2969: F, t310: F, t3010: F, t320: F, t315: F, t11132: F, t11337: F, t963: F, t3013: F, t323: F, t1034: F, t360: F, t11244: F, t11240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11450 = t302 * t11449;
    let t11452 = 1.0 / t2969 / t310;
    let t11465 = 1.0 / t3010 / t320;
    let t11466 = t315 * t11465;
    let t11479 = 0.93932222222222222223e0 * t11132;
    let t11480 = 0.36793333333333333333e0 * t11337;
    let t11506 = 1.0 / t3010 / t963;
    let t11507 = t315 * t11506;
    let t11509 = 1.0 / t3013 / t323;
    let t11534 = 0.55403703703703703703e-1 * t11132;
    let t11560 = 0.28842592592592592592e-1 * t11132;
    let t11574 = 0.53272592592592592592e-1 * t11132;
    let t11626 = t1034 * t1034;
    let t11627 = 1.0 / t11626;
    let t11628 = t11627 * t360;
    let t11629 = t11628 * t11244;
    let t11630 = t11240 * t11629;
    (t11450, t11452, t11465, t11466, t11479, t11480, t11506, t11507, t11509, t11534, t11560, t11574, t11627, t11630)
}
