//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1322/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1322<F: Float>(t1882: F, t31869: F, t31774: F, t10447: F, t10696: F, t112898: F, t112904: F, t11593: F, t125665: F, t125668: F, t15133: F, t15191: F, t15312: F, t1901: F, t19455: F, t19563: F, t24873: F, t28859: F, t29128: F, t29260: F, t296: F, t31747: F, t4176: F, t4181: F, t446: F, t7105: F, t7124: F, t840: F, t98788: F, t98790: F) -> (F,) {
    let t126088 = t1882 * t31869;
    let t126098 = t1882 * t31774;
    let t126111 = -t112898 + 16.0 / 27.0 * t112904 + 4.0 / 3.0 * t446 * t296 * t125668 - 4.0 / 9.0 * t1901 * t15312 * t24873 * t19563 + 8.0 / 9.0 * t11593 * t15312 * t24873 * t19455 - 4.0 * t1901 * t29128 * t10696 * t7124 * t4181 - 2.0 / 9.0 * t126088 + 2.0 / 3.0 * t446 * t840 * t28859 * t4176 + 2.0 / 3.0 * t446 * t840 * t15133 * t7105 + 2.0 / 81.0 * t126098 + 2.0 / 9.0 * t1901 * t15191 * t29260 + 4.0 / 27.0 * t98788 + 8.0 / 27.0 * t98790 + 4.0 / 3.0 * t446 * t296 * t125665 - 2.0 / 9.0 * t1901 * t10447 * t31747;
    (t126111,)
}
