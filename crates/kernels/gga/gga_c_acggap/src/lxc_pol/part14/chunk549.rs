//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 549/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk549<F: Float>(t1095: F, t398: F, t5099: F, t1036: F, t1032: F, t1434: F, t1539: F, t360: F, t1181: F, t1532: F, t1163: F, t372: F, t1165: F, t1552: F, t4210: F, t4241: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5101 = t398 * t1095 * t5099;
    let t5102 = t1036 * t5101;
    let t5104 = t1032 * t1434;
    let t5122 = t1539 * t360;
    let t5124 = t1181 * t1532 * t5122;
    let t5126 = 0.85748036236139473944e-3 * t1163 * t5124;
    let t5127 = t1539 * t372;
    let t5129 = t1165 * t1552 * t5127;
    let t5131 = 0.85748036236139473944e-3 * t1163 * t5129;
    let t5133 = t1165 * t1532 * t4210;
    let t5135 = 0.42874018118069736972e-3 * t1163 * t5133;
    let t5147 = t1165 * t1532 * t4241;
    (t5101, t5102, t5104, t5122, t5124, t5126, t5127, t5129, t5131, t5133, t5135, t5147)
}
