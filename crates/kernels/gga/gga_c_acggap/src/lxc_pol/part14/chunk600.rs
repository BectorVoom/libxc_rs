//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 600/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk600<F: Float>(t1016: F, t495: F, t1165: F, t1460: F, t1432: F, t4643: F, t1181: F, t3391: F, t407: F, t5852: F, t1524: F, t157: F, t1552: F, t1539: F, t1163: F, t1175: F, t5862: F) -> (F, F, F, F, F, F, F, F) {
    let t6138 = t1016 * t495;
    let t6140 = t1165 * t6138 * t1460;
    let t6143 = t4643 * t1432;
    let t6144 = t1181 * t6143;
    let t6145 = t3391 * t6144;
    let t6148 = t1165 * t5852 * t407;
    let t6151 = t157 * t1524;
    let t6153 = t1165 * t1552 * t6151;
    let t6157 = t1165 * t5852 * t1539;
    let t6158 = t1163 * t6157;
    let t6161 = t1165 * t5862 * t1175;
    (t6140, t6144, t6145, t6148, t6153, t6157, t6158, t6161)
}
