//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1297/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1297<F: Float>(t1173: F, t6837: F, t24237: F, t30879: F, t1403: F, t30899: F, t681: F, t107885: F, t107886: F, t107893: F, t109731: F, t109799: F, t1131: F, t122118: F, t122658: F, t122667: F, t124658: F, t17785: F, t17790: F, t17794: F, t18139: F, t193: F, t2354: F, t263: F, t4003: F, t6002: F, t6008: F, t684: F) -> (F,) {
    let t125220 = t6837 * t1173;
    let t125236 = t24237 * t30879;
    let t125239 = t1403 * t681 * t30899;
    let t125241 = -4.0 / 81.0 * t109731 - 2.0 * t124658 - 2.0 * t122118 - 2.0 / 3.0 * t1403 * t193 * t6008 * t4003 * t1131 - t1403 * t193 * t6008 * t263 * t18139 / 3.0 - t6002 * t2354 * t125220 * t684 / 9.0 - 2.0 / 3.0 * t107885 * t107893 * t17785 - 4.0 / 9.0 * t107885 * t107886 * t17790 + 4.0 / 27.0 * t107885 * t109799 * t17794 + 4.0 * t122667 + 4.0 * t122658 + t125236 / 27.0 - t125239 / 9.0;
    (t125241,)
}
