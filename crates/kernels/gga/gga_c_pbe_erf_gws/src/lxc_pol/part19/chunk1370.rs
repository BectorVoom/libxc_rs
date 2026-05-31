//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1370/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1370<F: Float>(t1144: F, t14886: F, t4386: F, t12255: F, t14182: F, t14188: F, t14193: F, t14881: F, t15036: F, t2408: F, t29775: F, t34922: F, t35057: F, t36323: F, t39689: F, t54957: F, t54978: F, t55065: F, t55344: F, t55345: F, t56773: F, t6793: F, t8629: F, t8793: F, t9283: F) -> F {
    let t58384 = t4386 * t1144 * t14886;
    let t58410 = t34922 * t14193 / F::cast_from(96.0_f64) + t6793 * t58384 / F::cast_from(24.0_f64) + t36323 * t14188 / F::cast_from(48.0_f64) + t36323 * t14182 / F::cast_from(48.0_f64) + t39689 * t14182 / F::cast_from(48.0_f64) + t8793 * t55065 / F::cast_from(24.0_f64) + t39689 * t14188 / F::cast_from(48.0_f64) + t29775 * t15036 / F::cast_from(24.0_f64) + t35057 * t14188 / F::cast_from(48.0_f64) + t8629 * t54978 / F::cast_from(48.0_f64) + t8793 * t54957 / F::cast_from(24.0_f64) + t2408 * t9283 * t14881 * t12255 / F::cast_from(8.0_f64) + t56773 / F::cast_from(48.0_f64) - t55344 + t55345;
    t58410
}
