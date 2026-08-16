//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 733/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk733<F: Float>(t353: F, t4228: F, t338: F, t1115: F, t2408: F, t3066: F, t335: F, t4072: F, t4077: F, t4083: F, t4087: F, t4128: F, t4131: F, t4133: F, t4136: F, t4139: F, t4143: F, t4147: F, t4151: F, t4209: F, t4213: F, t4218: F) -> (F, F) {
    let t4229 = t353 * t4228;
    let t4230 = t338 * t4229;
    let t4233 = t4128 / F::cast_from(48.0_f64) - t4072 - t4131 / F::cast_from(24.0_f64) + t4133 / F::cast_from(48.0_f64) - t4136 / F::cast_from(48.0_f64) + t4139 / F::cast_from(768.0_f64) - t4077 - t4143 / F::cast_from(384.0_f64) - t4147 / F::cast_from(1536.0_f64) - t4151 / F::cast_from(1536.0_f64) - t1115 * t4083 / F::cast_from(96.0_f64) + t4087 + t2408 * t4209 / F::cast_from(48.0_f64) - t335 * t4213 / F::cast_from(96.0_f64) + t3066 * t4218 / F::cast_from(48.0_f64) - t335 * t4230 / F::cast_from(96.0_f64);
    (t4230, t4233)
}
