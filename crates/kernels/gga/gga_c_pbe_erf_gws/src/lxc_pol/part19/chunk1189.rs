//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1189/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1189<F: Float>(t14229: F, t14233: F, t14551: F, t14554: F, t14558: F, t14563: F, t15249: F, t15251: F, t15253: F, t15256: F, t15258: F, t15260: F, t15262: F, t15264: F, t15266: F, t15269: F) -> F {
    let t15481 = -t15249 / F::cast_from(48.0_f64) - t15251 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t15253 + t15256 / F::cast_from(24.0_f64) + t15258 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14551 - t15260 / F::cast_from(24.0_f64) + t15262 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t14554 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14558 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t14563 + t14229 + t14233 - t15264 / F::cast_from(96.0_f64) + t15266 / F::cast_from(192.0_f64) - t15269 / F::cast_from(48.0_f64);
    t15481
}
