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
    let t58962 = t57719 / F::new(192.0) + t57731 / F::new(1536.0) - t2408 * t9283 * t52191 * t3742 / F::new(12.0) - t57740 / F::new(1536.0) + t57745 / F::new(768.0) - t57747 / F::new(8.0) + t57755 / F::new(96.0) + t55983 - F::new(7.0) / F::new(288.0) * t58951 - t35193 * t4083 / F::new(96.0) - F::new(35.0) / F::new(54.0) * t54719 - t335 * t338 * t1144 * t15082 / F::new(48.0) - F::new(119.0) / F::new(3456.0) * t54724 - t57764 / F::new(1536.0) + t55987 + t54737;
    t58962
}
