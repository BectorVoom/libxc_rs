//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1256/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1256<F: Float>(t1882: F, t29342: F, t38953: F, t7033: F, t56456: F, t6360: F, t7102: F, t11593: F, t14116: F, t14678: F, t15294: F, t15303: F, t15442: F, t15523: F, t1901: F, t24886: F, t2682: F, t2862: F, t29207: F, t4129: F, t446: F, t53797: F, t6393: F, t7131: F, t840: F, t98924: F, t98926: F, t98933: F, t98940: F, t98942: F, t98966: F) -> (F,) {
    let t113773 = 2.0 / 9.0 * t1882 * t29342;
    let t113778 = t38953 * t7033;
    let t113780 = t56456 * t6360;
    let t113800 = t38953 * t7102;
    let t113802 = 2.0 / 3.0 * t446 * t2862 * t7131 * t2682 - 2.0 / 27.0 * t98924 + 2.0 / 27.0 * t98926 + t113773 - 2.0 / 3.0 * t446 * t840 * t6393 * t4129 + 4.0 / 81.0 * t113778 + 4.0 / 9.0 * t53797 * t113780 * t14678 + 4.0 / 9.0 * t53797 * t98966 * t15303 + 2.0 / 27.0 * t98933 + t1901 * t24886 * t15523 / 9.0 - 4.0 / 9.0 * t11593 * t24886 * t15442 - 8.0 / 27.0 * t98940 - 8.0 / 27.0 * t98942 - 8.0 / 27.0 * t11593 * t15294 * t29207 * t14116 + 4.0 / 81.0 * t113800;
    (t113802,)
}
