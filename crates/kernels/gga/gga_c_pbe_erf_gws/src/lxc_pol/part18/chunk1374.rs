//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1374/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1374<F: Float>(t2409: F, t35890: F, t3965: F, t12243: F, t14121: F, t13772: F, t3200: F, t335: F, t338: F, t3917: F, t4183: F, t51957: F, t54536: F, t54538: F, t54567: F, t57581: F, t57584: F, t57588: F, t57593: F, t57595: F, t57598: F, t57602: F, t57605: F, t6793: F) -> F {
    let t57608 = t3965 * t2409 * t35890;
    let t57614 = t14121 * t12243;
    let t57618 = -t54536 + t54538 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57581 + t57584 / F::cast_from(768.0_f64) + t6793 * t57588 / F::cast_from(48.0_f64) + t57593 / F::cast_from(768.0_f64) + t57595 / F::cast_from(24.0_f64) - t57598 / F::cast_from(48.0_f64) + t54567 - t57602 / F::cast_from(384.0_f64) - t57605 / F::cast_from(48.0_f64) - t57608 / F::cast_from(96.0_f64) + t51957 - t335 * t338 * t3200 * t4183 / F::cast_from(48.0_f64) + t57614 / F::cast_from(16.0_f64) - t3917 * t13772 / F::cast_from(96.0_f64);
    t57618
}
