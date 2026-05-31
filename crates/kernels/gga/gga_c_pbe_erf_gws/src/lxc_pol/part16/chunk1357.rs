//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1357/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1357<F: Float>(t54641: F, t54681: F, t1144: F, t1206: F, t14241: F, t14311: F, t14881: F, t22263: F, t3066: F, t335: F, t338: F, t4083: F, t51992: F, t52542: F, t52586: F, t52589: F, t52603: F, t54649: F, t54664: F, t54675: F, t8654: F, t8793: F, t9201: F, t9283: F, t9321: F) -> F {
    let t55947 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t54641;
    let t55962 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t54681;
    let t55973 = t55947 - t335 * t338 * t9201 * t1206 / F::cast_from(96.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t52586 - t335 * t338 * t1144 * t14241 / F::cast_from(96.0_f64) + t52589 - t54649 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51992 + t54664 / F::cast_from(12.0_f64) - t54675 / F::cast_from(12.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t52603 - t55962 - t3066 * t9283 * t14881 * t9321 / F::cast_from(16.0_f64) + t8793 * t52542 / F::cast_from(24.0_f64) - t22263 * t4083 / F::cast_from(48.0_f64) - t8654 * t14311 / F::cast_from(48.0_f64);
    t55973
}
