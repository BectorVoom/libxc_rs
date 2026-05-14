//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1356/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1356<F: Float>(t19191: F, t2380: F, t3899: F, t10084: F, t3206: F, t926: F, t10252: F, t2099: F, t3235: F, t2411: F, t9795: F, t10189: F, t2029: F, t3880: F, t5728: F, t10225: F, t18657: F) -> (F, F, F, F, F, F, F) {
    let t27073 = t2380 * t19191 * t3899;
    let t27076 = t3206 * t926 * t10084;
    let t27083 = t3235 * t2099 * t10252;
    let t27085 = t2411 * t9795;
    let t27104 = t10189 * t2029;
    let t27113 = t3880 * t5728;
    let t27119 = t2380 * t18657 * t10225;
    (t27073, t27076, t27083, t27085, t27104, t27113, t27119)
}
