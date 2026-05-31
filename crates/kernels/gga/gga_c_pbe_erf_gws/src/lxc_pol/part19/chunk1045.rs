//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1045/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1045<F: Float>(t11737: F, t2300: F, t904: F, t11680: F, t11685: F, t11689: F, t11695: F, t11699: F, t11701: F, t11734: F, t2277: F, t2312: F, t8960: F, t8969: F, t8971: F, t8973: F, t914: F, t929: F, t9498: F) -> (F, F) {
    let t11739 = t2300 * t904 * t11737;
    let t11742 = t8960 - t2312 * t11680 / F::cast_from(192.0_f64) - t2312 * t11685 / F::cast_from(192.0_f64) - t2277 * t11689 / F::cast_from(384.0_f64) - t11695 + t11699 - t8969 - t2312 * t11701 / F::cast_from(384.0_f64) + t8971 - t914 * t11734 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t929 * t11739 + t9498 + t8973;
    (t11739, t11742)
}
