//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 638/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk638<F: Float>(t1165: F, t1552: F, t6151: F, t1539: F, t5852: F, t1163: F, t1175: F, t5862: F, t1140: F, t1784: F, t336: F, t337: F, t5506: F) -> (F, F, F, F, F, F) {
    let t6153 = t1165 * t1552 * t6151;
    let t6157 = t1165 * t5852 * t1539;
    let t6158 = t1163 * t6157;
    let t6161 = t1165 * t5862 * t1175;
    let t6164 = t1140 * t1784;
    let t6167 = t336 * t337 * t5506;
    (t6153, t6157, t6158, t6161, t6164, t6167)
}
