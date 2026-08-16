//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1339/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1339<F: Float>(t1144: F, t13930: F, t14107: F, t29775: F, t335: F, t338: F, t4002: F, t51592: F, t51599: F, t51604: F, t54541: F, t54545: F, t54550: F, t54561: F, t54564: F, t54567: F, t54572: F, t54575: F, t54581: F, t6793: F, t8616: F, t8793: F) -> F {
    let t54583 = t54541 / F::cast_from(1536.0_f64) + t6793 * t54545 / F::cast_from(24.0_f64) + t6793 * t54550 / F::cast_from(24.0_f64) + t29775 * t13930 / F::cast_from(24.0_f64) + t8793 * t51592 / F::cast_from(24.0_f64) + t8793 * t51599 / F::cast_from(24.0_f64) + t8793 * t51604 / F::cast_from(48.0_f64) + t54561 / F::cast_from(96.0_f64) - t54564 / F::cast_from(96.0_f64) + t54567 - t335 * t338 * t1144 * t14107 / F::cast_from(96.0_f64) + t54572 / F::cast_from(48.0_f64) - t54575 / F::cast_from(48.0_f64) - t8616 * t4002 / F::cast_from(96.0_f64) - t54581 / F::cast_from(32.0_f64);
    t54583
}
