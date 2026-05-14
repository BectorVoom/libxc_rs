//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 877/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk877<F: Float>(t1658: F, t449: F, t863: F, t864: F, t1659: F, t3896: F, t4109: F, t857: F, t1265: F, t4137: F, t1614: F, t3858: F, t12224: F, t557: F, t1605: F, t848: F) -> (F, F, F, F, F, F, F) {
    let t15214 = t863 * t449 * t1658 * t864;
    let t15218 = t3896 * t1659;
    let t15221 = t857 * t4109;
    let t15223 = t4137 * t1265;
    let t15230 = t3858 * t1614;
    let t15232 = t12224 * t557;
    let t15234 = t848 * t1605;
    (t15214, t15218, t15221, t15223, t15230, t15232, t15234)
}
