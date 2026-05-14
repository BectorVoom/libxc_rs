//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 500/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk500<F: Float>(t3429: F, t571: F, t11: F, t1856: F, t3421: F, t3425: F, t606: F, t1844: F, t1851: F, t25: F, t2760: F, t2773: F, t3423: F, t3427: F) -> (F, F, F, F, F, F) {
    let t3430 = t571 * t3429;
    let t3431 = t11 * t3430;
    let t3434 = t1856 * t3421;
    let t3437 = t606 * t3425;
    let t3440 = t606 * t3429;
    let t3443 = t1844 + 0.23994444444444444444e-1 * t2760 - 0.23994444444444444445e-1 * t3423 + 0.71983333333333333334e-1 * t3427 - 0.35991666666666666667e-1 * t3431 + t1851 + 0.8888888888888888889e-2 * t2773 - 0.22222222222222222222e-2 * t25 * t3434 + 0.13333333333333333333e-1 * t25 * t3437 - 0.66666666666666666667e-2 * t25 * t3440;
    (t3430, t3431, t3434, t3437, t3440, t3443)
}
