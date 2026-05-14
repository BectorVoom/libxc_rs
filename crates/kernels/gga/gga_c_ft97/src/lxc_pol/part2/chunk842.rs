//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 842/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk842<F: Float>(t14895: F, t14902: F, t10243: F, t10658: F, t14718: F, t14892: F, t14899: F, t15058: F, t15062: F, t15065: F, t15069: F, t15087: F, t15099: F, t15112: F, t312: F, t1240: F, t2842: F) -> (F, F, F) {
    let t15116 = 4.0 / 27.0 * t14895;
    let t15118 = 2.0 / 9.0 * t14902;
    let t15123 = -22.0 / 27.0 * t14718 - 2.0 / 27.0 * t10243 - t14892 / 3.0 - t15116 + 2.0 / 9.0 * t14899 + t15118 + t15058 / 6.0 - t10658 - t15062 / 6.0 - t15065 / 12.0 + t15069 / 8.0;
    let t15125 = t15087 + t15099 + t15112 + t15123;
    let t15126 = t15125 * t312;
    let t15128 = t1240 * t2842;
    (t15125, t15126, t15128)
}
