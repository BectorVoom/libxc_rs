//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 955/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk955<F: Float>(t3868: F, t5351: F, t1658: F, t449: F, t863: F, t864: F, t1659: F, t3896: F, t4109: F, t857: F, t1265: F, t4137: F) -> (F, F, F, F, F) {
    let t15210 = t3868 * t5351;
    let t15214 = t863 * t449 * t1658 * t864;
    let t15218 = t3896 * t1659;
    let t15221 = t857 * t4109;
    let t15223 = t4137 * t1265;
    (t15210, t15214, t15218, t15221, t15223)
}
