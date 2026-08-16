//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1365/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1365<F: Float>(t1109: F, t14918: F, t15036: F, t15535: F, t15537: F, t22379: F, t2408: F, t26604: F, t3212: F, t353: F, t4111: F, t54962: F, t55151: F, t55154: F, t55238: F, t55243: F, t55284: F, t55698: F, t56582: F, t56586: F, t56588: F, t56590: F, t859: F, t8629: F, t8654: F, t8793: F, t892: F, t9283: F) -> F {
    let t58257 = t56582 / F::cast_from(384.0_f64) - t56586 / F::cast_from(192.0_f64) - t2408 * t9283 * t55151 * t3212 / F::cast_from(12.0_f64) - t55238 + t56588 / F::cast_from(48.0_f64) + t56590 / F::cast_from(48.0_f64) + F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t55243 - t8654 * t14918 / F::cast_from(48.0_f64) + t22379 * t15036 / F::cast_from(24.0_f64) + t8629 * t55698 / F::cast_from(48.0_f64) + t8793 * t54962 / F::cast_from(24.0_f64) + t8793 * t55284 / F::cast_from(24.0_f64) + t8793 * t55154 / F::cast_from(24.0_f64) + t26604 * t15537 / F::cast_from(96.0_f64) + t8629 * t859 * t892 * t15535 / F::cast_from(96.0_f64) + t8629 * t859 * t353 * t4111 * t1109 / F::cast_from(96.0_f64);
    t58257
}
