//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1162/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1162<F: Float>(t14655: F, t4218: F, t9270: F, t14295: F, t14302: F, t14305: F, t14634: F, t14640: F, t14649: F, t14658: F, t14945: F, t14949: F, t14954: F, t14959: F, t2408: F, t3066: F) -> (F, F) {
    let t14962 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t14655;
    let t14964 = t9270 * t4218;
    let t14967 = t14634 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t14640 + t3066 * t14945 / F::cast_from(48.0_f64) + t3066 * t14949 / F::cast_from(48.0_f64) + t2408 * t14954 / F::cast_from(48.0_f64) - t14649 / F::cast_from(48.0_f64) - t2408 * t14959 / F::cast_from(24.0_f64) + t14295 + t14962 - t14302 - t14658 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14964 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14305;
    (t14964, t14967)
}
