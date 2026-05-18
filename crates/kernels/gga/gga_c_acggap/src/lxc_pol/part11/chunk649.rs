//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 649/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk649<F: Float>(t1539: F, t372: F, t1165: F, t1552: F, t1163: F, t1532: F, t4210: F, t1533: F, t360: F, t1181: F, t4241: F, t3456: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5127 = t1539 * t372;
    let t5129 = t1165 * t1552 * t5127;
    let t5131 = F::new(0.85748036236139473944e-3) * t1163 * t5129;
    let t5133 = t1165 * t1532 * t4210;
    let t5135 = F::new(0.42874018118069736972e-3) * t1163 * t5133;
    let t5136 = t1533 * t360;
    let t5138 = t1181 * t1532 * t5136;
    let t5141 = t1533 * t372;
    let t5143 = t1165 * t1552 * t5141;
    let t5147 = t1165 * t1532 * t4241;
    let t5149 = F::new(0.85748036236139473944e-3) * t3456 * t5147;
    (t5127, t5129, t5131, t5133, t5135, t5136, t5138, t5141, t5143, t5147, t5149)
}
