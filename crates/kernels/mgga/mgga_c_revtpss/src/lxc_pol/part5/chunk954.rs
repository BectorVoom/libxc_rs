//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 954/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk954<F: Float>(t2967: F, t941: F, t2966: F, t307: F, t302: F, t11132: F, t11337: F, t944: F, t2969: F, t310: F, t3011: F, t960: F, t3010: F, t320: F, t315: F, t963: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11404 = t941 * t2967;
    let t11408 = 1.0 / t2966 / t307;
    let t11409 = t302 * t11408;
    let t11422 = 0.16068111111111111111e1 * t11132;
    let t11423 = 0.46308888888888888888e0 * t11337;
    let t11449 = 1.0 / t2966 / t944;
    let t11450 = t302 * t11449;
    let t11452 = 1.0 / t2969 / t310;
    let t11461 = t960 * t3011;
    let t11465 = 1.0 / t3010 / t320;
    let t11466 = t315 * t11465;
    let t11479 = 0.93932222222222222223e0 * t11132;
    let t11480 = 0.36793333333333333333e0 * t11337;
    let t11506 = 1.0 / t3010 / t963;
    (t11404, t11409, t11422, t11423, t11450, t11452, t11461, t11465, t11466, t11479, t11480, t11506)
}
