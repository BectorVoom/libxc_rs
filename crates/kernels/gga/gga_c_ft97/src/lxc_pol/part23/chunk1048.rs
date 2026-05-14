//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1048/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1048<F: Float>(t4969: F, t6273: F, t2874: F, t24886: F, t5414: F, t1255: F, t7021: F, t840: F, t15191: F, t7032: F, t1091: F, t29259: F, t4973: F, t4965: F, t10479: F, t6360: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31720 = t6273 * t4969;
    let t31721 = t2874 * t31720;
    let t31724 = t24886 * t5414;
    let t31729 = t840 * t1255 * t7021;
    let t31732 = t15191 * t7032;
    let t31735 = t29259 * t1091;
    let t31736 = t2874 * t31735;
    let t31739 = t6273 * t4973;
    let t31740 = t2874 * t31739;
    let t31743 = t6273 * t4965;
    let t31744 = t10479 * t31743;
    let t31747 = t6360 * t4969;
    (t31720, t31721, t31724, t31729, t31732, t31735, t31736, t31739, t31740, t31743, t31744, t31747)
}
