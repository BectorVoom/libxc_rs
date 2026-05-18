//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 636/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk636<F: Float>(t1758: F, t3421: F, t11: F, t1764: F, t3342: F, t571: F, t3346: F, t572: F, t1856: F, t606: F, t1844: F, t1851: F, t25: F, t2760: F, t2773: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3422 = t1758 * t3421;
    let t3423 = t11 * t3422;
    let t3425 = t1764 * t3342;
    let t3426 = t571 * t3425;
    let t3427 = t11 * t3426;
    let t3429 = t572 * t3346;
    let t3430 = t571 * t3429;
    let t3431 = t11 * t3430;
    let t3434 = t1856 * t3421;
    let t3437 = t606 * t3425;
    let t3440 = t606 * t3429;
    let t3443 = t1844 + F::new(0.23994444444444444444e-1) * t2760 - F::new(0.23994444444444444445e-1) * t3423 + F::new(0.71983333333333333334e-1) * t3427 - F::new(0.35991666666666666667e-1) * t3431 + t1851 + F::new(0.8888888888888888889e-2) * t2773 - F::new(0.22222222222222222222e-2) * t25 * t3434 + F::new(0.13333333333333333333e-1) * t25 * t3437 - F::new(0.66666666666666666667e-2) * t25 * t3440;
    (t3422, t3423, t3425, t3426, t3427, t3429, t3430, t3431, t3434, t3437, t3440, t3443)
}
