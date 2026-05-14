//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1174/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1174<F: Float>(t113631: F, t1882: F, t28521: F, t7091: F, t848: F, t29147: F, t8392: F, t28719: F, t312: F, t29094: F, t29261: F, t29216: F, t29342: F, t38953: F, t7033: F, t56456: F, t6360: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113632 = 4.0 / 9.0 * t113631;
    let t113633 = t1882 * t28521;
    let t113634 = 4.0 / 9.0 * t113633;
    let t113656 = t848 * t7091;
    let t113665 = 2.0 / 27.0 * t8392 * t29147;
    let t113710 = t312 * t28719;
    let t113716 = 4.0 / 81.0 * t8392 * t29094;
    let t113722 = 2.0 / 27.0 * t8392 * t29261;
    let t113749 = 2.0 / 27.0 * t8392 * t29216;
    let t113773 = 2.0 / 9.0 * t1882 * t29342;
    let t113778 = t38953 * t7033;
    let t113780 = t56456 * t6360;
    (t113632, t113633, t113634, t113656, t113665, t113710, t113716, t113722, t113749, t113773, t113778, t113780)
}
