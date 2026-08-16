//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1312/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1312<F: Float>(t1144: F, t14191: F, t859: F, t14180: F, t4386: F, t14949: F, t9270: F, t14943: F, t14979: F, t15025: F, t2408: F, t2409: F, t3066: F, t4385: F, t50995: F, t52183: F, t53131: F, t53134: F, t53140: F, t53152: F, t53158: F, t53166: F, t53170: F, t6781: F, t6793: F, t8734: F) -> F {
    let t54978 = t859 * t1144 * t14191;
    let t54984 = t4386 * t1144 * t14180;
    let t54998 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t14949;
    let t55003 = -t53131 / F::cast_from(768.0_f64) + t53134 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t50995 - t53140 / F::cast_from(192.0_f64) + t53152 / F::cast_from(192.0_f64) + t4385 * t54978 / F::cast_from(96.0_f64) - t53158 / F::cast_from(48.0_f64) - t53166 / F::cast_from(192.0_f64) + t6793 * t54984 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t52183 + t2408 * t2409 * t6781 * t14979 / F::cast_from(24.0_f64) + t3066 * t2409 * t8734 * t15025 / F::cast_from(24.0_f64) + t53170 / F::cast_from(192.0_f64) - t54998 + t3066 * t2409 * t8734 * t14943 / F::cast_from(24.0_f64);
    t55003
}
