//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1401/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1401<F: Float>(t15537: F, t22493: F, t1144: F, t15082: F, t2408: F, t335: F, t338: F, t35193: F, t3742: F, t4083: F, t52191: F, t54719: F, t54724: F, t54737: F, t55983: F, t55987: F, t57719: F, t57731: F, t57740: F, t57745: F, t57747: F, t57755: F, t57764: F, t9283: F) -> F {
    let t58951 = t22493 * t15537;
    let t58962 = t57719 / F::cast_from(192.0_f64) + t57731 / F::cast_from(1536.0_f64) - t2408 * t9283 * t52191 * t3742 / F::cast_from(12.0_f64) - t57740 / F::cast_from(1536.0_f64) + t57745 / F::cast_from(768.0_f64) - t57747 / F::cast_from(8.0_f64) + t57755 / F::cast_from(96.0_f64) + t55983 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t58951 - t35193 * t4083 / F::cast_from(96.0_f64) - F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t54719 - t335 * t338 * t1144 * t15082 / F::cast_from(48.0_f64) - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t54724 - t57764 / F::cast_from(1536.0_f64) + t55987 + t54737;
    t58962
}
