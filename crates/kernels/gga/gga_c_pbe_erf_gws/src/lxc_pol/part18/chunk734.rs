//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 734/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk734<F: Float>(t353: F, t4183: F, t338: F, t1115: F, t2408: F, t3066: F, t335: F, t3957: F, t3981: F, t4002: F, t4006: F, t4128: F, t4131: F, t4133: F, t4136: F, t4139: F, t4143: F, t4147: F, t4151: F, t4157: F, t4161: F, t4166: F) -> (F, F) {
    let t4184 = t353 * t4183;
    let t4185 = t338 * t4184;
    let t4188 = t4128 / F::new(96.0) - t3957 - t4131 / F::new(48.0) + t4133 / F::new(96.0) - t4136 / F::new(96.0) + t4139 / F::new(1536.0) - t3981 - t4143 / F::new(768.0) - t4147 / F::new(3072.0) - t4151 / F::new(3072.0) - t1115 * t4002 / F::new(96.0) + t4006 + t2408 * t4157 / F::new(48.0) - t335 * t4161 / F::new(96.0) + t3066 * t4166 / F::new(48.0) - t335 * t4185 / F::new(96.0);
    (t4185, t4188)
}
