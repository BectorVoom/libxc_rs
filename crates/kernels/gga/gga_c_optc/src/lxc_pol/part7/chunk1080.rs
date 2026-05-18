//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1080/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1080<F: Float>(t188: F, t202: F, t23047: F, t6529: F, t740: F, t108: F, t176: F, t185: F, t203: F, t22052: F, t22700: F, t22703: F, t22705: F, t22708: F, t22711: F, t22713: F, t22716: F, t22719: F) -> F {
    let t23431 = F::new(7280.0) / F::new(81.0) * t188 * t23047 * t202;
    let t23432 = t6529 * t740;
    let t23434 = t22700 + t22703 + t22705 + t22708 - t22711 - t22713 - t22716 - t22719 + t176 * t185 * t22052 * t108 * t203 / F::new(2.0) + t23431 - F::new(14.0) * t23432;
    t23434
}
