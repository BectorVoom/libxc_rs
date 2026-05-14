//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 981/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk981<F: Float>(t12813: F, t5218: F, t7495: F, t31879: F, t1019: F, t12486: F, t1046: F, t12528: F, t18215: F, t47760: F, t47761: F, t47762: F, t47765: F, t47769: F, t47772: F, t24131: F) -> (F, F, F, F, F, F) {
    let t47775 = 64.0 / 15.0 * t5218 * t7495 * t12813;
    let t47776 = 16.0 / 45.0 * t31879;
    let t47778 = 8.0 / 15.0 * t12486 * t1019;
    let t47780 = 16.0 / 5.0 * t12528 * t1046;
    let t47781 = t47760 - t47761 + t47762 + t47765 - t47769 + t47772 + t47775 + t18215 + t47776 - t47778 - t47780;
    let t47782 = 64.0 / 405.0 * t24131;
    (t47775, t47776, t47778, t47780, t47781, t47782)
}
