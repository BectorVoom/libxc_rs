//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 945/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk945<F: Float>(t1730: F, t5171: F, t5163: F, t582: F, t616: F, t5004: F, t5480: F, t639: F, t1631: F, t5470: F, t1627: F, t5477: F) -> (F, F, F, F, F) {
    let t17558 = t1730 * t5171;
    let t17559 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t17558;
    let t17561 = t616 * t582 * t5163;
    let t17562 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17561;
    let t17564 = t639 * t5480 * t5004;
    let t17565 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t17564;
    let t17566 = t5470 * t1631;
    let t17567 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17566;
    let t17568 = t1627 * t5477;
    (t17559, t17562, t17565, t17567, t17568)
}
