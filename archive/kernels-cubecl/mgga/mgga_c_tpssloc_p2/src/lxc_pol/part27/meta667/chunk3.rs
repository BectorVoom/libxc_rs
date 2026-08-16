//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2346/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2346<F: Float>(t16123: F, t2002: F, t559: F, t80920: F, t80922: F, t80943: F, t80957: F, t80959: F, t80971: F, t80989: F, t80992: F, t80998: F, t81007: F, t91394: F, t91398: F, t91400: F, t91403: F, t91404: F, t91406: F, t91413: F) -> F {
    let t91416 = t16123 * t2002 * t559;
    let t91418 = -F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t91394 + F::cast_from(0.14130464632949136799e-2_f64) * t80920 + F::cast_from(0.14130464632949136799e-2_f64) * t80922 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t91398 - F::cast_from(0.67826230238155856634e-1_f64) * t91400 + t91403 + F::cast_from(0.16956557559538964158e-1_f64) * t91404 - t91406 - F::cast_from(0.28260929265898273598e-2_f64) * t80943 - t80957 - F::cast_from(0.16956557559538964159e-1_f64) * t80959 + t80971 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t80989 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t80992 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t80998 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t81007 + t91413 / F::cast_from(192.0_f64) + t91416 / F::cast_from(1536.0_f64);
    t91418
}
