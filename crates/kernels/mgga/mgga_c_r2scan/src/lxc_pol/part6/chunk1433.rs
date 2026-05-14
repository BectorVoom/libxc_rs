//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1433/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1433<F: Float>(t19421: F, t19424: F, t22630: F, t22633: F, t22636: F, t23922: F, t23927: F, t23928: F, t23929: F, t23976: F, t26964: F, t26965: F, t765: F, t19611: F, t19614: F, t19620: F, t19624: F, t19628: F, t19646: F, t19649: F, t19720: F, t22647: F, t23937: F, t23938: F) -> (F, F) {
    let t26972 = -t26964 - 0.675260332e-1 * t26965 + t19421 + t23922 + t19424 + t23927 + t23928 + 0.2025780996e0 * t765 * t23976 - t23929 + 0.285764e-1 * t22630 + 0.857292e-1 * t22633 + 0.857292e-1 * t22636;
    let t26975 = t22647 + t23937 + t23938 - t19720 - t19611 - t19614 + t19620 - t19624 + t19628 + t19646 + t19649;
    (t26972, t26975)
}
