//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1263/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1263<F: Float>(t54285: F, t54289: F, t54301: F, t54319: F, t54322: F, t54329: F, t54344: F, t54354: F, t54377: F, t54397: F, t54401: F, t14937: F, t9270: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t55570 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54285;
    let t55572 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54289;
    let t55580 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54301;
    let t55591 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t54319;
    let t55593 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t54322;
    let t55596 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t54329;
    let t55603 = F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t54344;
    let t55608 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54354;
    let t55620 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t54377;
    let t55633 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54397;
    let t55634 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54401;
    let t55660 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t14937;
    (t55570, t55572, t55580, t55591, t55593, t55596, t55603, t55608, t55620, t55633, t55634, t55660)
}
