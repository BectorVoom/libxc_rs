//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1382/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1382<F: Float>(t11363: F, t11407: F, t1193: F, t14791: F, t15337: F, t2408: F, t29751: F, t3066: F, t3742: F, t51084: F, t54667: F, t54682: F, t57694: F, t57696: F, t57700: F, t57702: F, t57705: F, t57707: F, t57711: F, t57719: F, t57731: F, t9241: F, t9283: F) -> F {
    let t57737 = t54667 - t57694 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t57696 - t57700 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57702 - t57705 / F::cast_from(24.0_f64) - t54682 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57707 + t57711 / F::cast_from(768.0_f64) + t9241 * t9283 * t1193 * t11363 / F::cast_from(4.0_f64) + t57719 / F::cast_from(384.0_f64) - t3066 * t9283 * t14791 * t11407 / F::cast_from(8.0_f64) - t2408 * t29751 * t15337 / F::cast_from(12.0_f64) + t57731 / F::cast_from(3072.0_f64) - t2408 * t9283 * t51084 * t3742 / F::cast_from(12.0_f64);
    t57737
}
