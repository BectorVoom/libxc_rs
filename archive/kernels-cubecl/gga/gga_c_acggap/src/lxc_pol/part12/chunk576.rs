//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 576/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk576<F: Float>(t1630: F, t3077: F, t1629: F, t955: F, t150: F, t2934: F, t119: F, t2937: F, t943: F, t945: F, t1651: F, t930: F) -> (F, F, F, F, F, F, F) {
    let t4192 = t3077 * t1630;
    let t4194 = t1629 * t955;
    let t4197 = t150 * t2934;
    let t4198 = t119 * t4197;
    let t4199 = t2937 * t943;
    let t4200 = t1629 * t4199;
    let t4203 = t1629 * t945;
    let t4206 = t1651 * t930;
    (t4192, t4194, t4198, t4199, t4200, t4203, t4206)
}
