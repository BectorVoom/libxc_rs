//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1143/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1143<F: Float>(t1144: F, t14191: F, t859: F, t14180: F, t4386: F, t14949: F, t9270: F, t14943: F, t14979: F, t15025: F, t2408: F, t2409: F, t3066: F, t4385: F, t50995: F, t52183: F, t53131: F, t53134: F, t53140: F, t53152: F, t53158: F, t53166: F, t53170: F, t6781: F, t6793: F, t8734: F) -> (F,) {
    let t54978 = t859 * t1144 * t14191;
    let t54984 = t4386 * t1144 * t14180;
    let t54998 = 7.0 / 72.0 * t9270 * t14949;
    let t55003 = -t53131 / 768.0 + t53134 / 24.0 + 7.0 / 144.0 * t50995 - t53140 / 192.0 + t53152 / 192.0 + t4385 * t54978 / 96.0 - t53158 / 48.0 - t53166 / 192.0 + t6793 * t54984 / 24.0 - 7.0 / 288.0 * t52183 + t2408 * t2409 * t6781 * t14979 / 24.0 + t3066 * t2409 * t8734 * t15025 / 24.0 + t53170 / 192.0 - t54998 + t3066 * t2409 * t8734 * t14943 / 24.0;
    (t55003,)
}
