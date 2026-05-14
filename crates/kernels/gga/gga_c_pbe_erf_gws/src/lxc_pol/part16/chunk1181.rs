//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1181/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1181<F: Float>(t54716: F, t54719: F, t54724: F, t54730: F, t14327: F, t52036: F, t52607: F, t54690: F, t54694: F, t54697: F, t54702: F, t54707: F, t54714: F, t54722: F, t54734: F, t8654: F) -> (F,) {
    let t55983 = 7.0 / 576.0 * t54716;
    let t55984 = 35.0 / 108.0 * t54719;
    let t55986 = 119.0 / 6912.0 * t54724;
    let t55987 = 7.0 / 576.0 * t54730;
    let t55990 = t54690 / 192.0 - t8654 * t14327 / 48.0 - t54694 / 192.0 + 7.0 / 288.0 * t52607 - t54697 / 96.0 + t54702 / 384.0 - t54707 / 384.0 + t54714 / 12.0 + t55983 - t55984 - t54722 / 24.0 - t55986 + t55987 - t54734 / 8.0 + 35.0 / 108.0 * t52036;
    (t55990,)
}
