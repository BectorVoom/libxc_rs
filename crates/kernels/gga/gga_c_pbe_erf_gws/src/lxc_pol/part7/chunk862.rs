//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 862/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk862<F: Float>(t1730: F, t5171: F, t5163: F, t582: F, t616: F, t5004: F, t5480: F, t639: F, t1631: F, t5470: F, t1627: F, t5477: F, t5481: F, t5164: F, t2730: F, t16745: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17558 = t1730 * t5171;
    let t17559 = 32.0 / 15.0 * t17558;
    let t17561 = t616 * t582 * t5163;
    let t17562 = 32.0 / 45.0 * t17561;
    let t17564 = t639 * t5480 * t5004;
    let t17565 = 64.0 / 27.0 * t17564;
    let t17566 = t5470 * t1631;
    let t17567 = 32.0 / 45.0 * t17566;
    let t17568 = t1627 * t5477;
    let t17569 = 32.0 / 45.0 * t17568;
    let t17570 = t1627 * t5481;
    let t17571 = 32.0 / 27.0 * t17570;
    let t17573 = 16.0 / 15.0 * t1730 * t5164;
    let t17575 = 16.0 / 15.0 * t2730 * t5164;
    let t17577 = -12.0 * t16745;
    (t17559, t17562, t17565, t17567, t17569, t17571, t17573, t17575, t17577)
}
