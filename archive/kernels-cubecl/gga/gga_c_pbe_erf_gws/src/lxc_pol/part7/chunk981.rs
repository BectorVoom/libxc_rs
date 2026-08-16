//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 981/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk981<F: Float>(t4551: F, t713: F, t1457: F, t1917: F, t762: F, t16487: F, t16490: F, t16503: F, t16508: F, t16512: F, t16515: F, t18146: F, t18149: F) -> F {
    let t18150 = t4551 * t713;
    let t18152 = t1457 * t713;
    let t18155 = F::cast_from(0.26596355555555555555e0_f64) * t762 * t1917;
    let t18156 = F::cast_from(0.39894533333333333332e0_f64) * t18146 + t18149 + F::cast_from(0.19947266666666666666e0_f64) * t18150 - F::cast_from(0.26596355555555555555e0_f64) * t18152 - t18155 - t16487 - t16490 - t16503 + t16508 + t16512 - t16515;
    t18156
}
