//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1382/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1382<F: Float>(t11363: F, t11407: F, t1193: F, t14791: F, t15337: F, t2408: F, t29751: F, t3066: F, t3742: F, t51084: F, t54667: F, t54682: F, t57694: F, t57696: F, t57700: F, t57702: F, t57705: F, t57707: F, t57711: F, t57719: F, t57731: F, t9241: F, t9283: F) -> F {
    let t57737 = t54667 - t57694 / F::new(24.0) + F::new(7.0) / F::new(72.0) * t57696 - t57700 / F::new(768.0) + F::new(7.0) / F::new(144.0) * t57702 - t57705 / F::new(24.0) - t54682 - F::new(7.0) / F::new(288.0) * t57707 + t57711 / F::new(768.0) + t9241 * t9283 * t1193 * t11363 / F::new(4.0) + t57719 / F::new(384.0) - t3066 * t9283 * t14791 * t11407 / F::new(8.0) - t2408 * t29751 * t15337 / F::new(12.0) + t57731 / F::new(3072.0) - t2408 * t9283 * t51084 * t3742 / F::new(12.0);
    t57737
}
