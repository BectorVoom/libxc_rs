//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 946/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk946<F: Float>(t17568: F, t1627: F, t5481: F, t1730: F, t5164: F, t2730: F, t16745: F, t186: F, t220: F, t616: F, t1726: F, t1750: F) -> (F, F, F, F, F, F, F) {
    let t17569 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17568;
    let t17570 = t1627 * t5481;
    let t17571 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t17570;
    let t17573 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1730 * t5164;
    let t17575 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t2730 * t5164;
    let t17577 = -F::cast_from(12.0_f64) * t16745;
    let t17581 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t616 * t186 * t220 * t17577;
    let t17583 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1750 * t1726;
    (t17569, t17571, t17573, t17575, t17577, t17581, t17583)
}
