//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1168/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1168<F: Float>(t33581: F, t33583: F, t22653: F, t48472: F, t85: F, t22655: F, t42535: F, t18885: F, t18941: F, t18950: F, t18954: F, t18956: F, t18959: F) -> (F, F, F, F, F, F, F) {
    let t48502 = F::new(72.0) * t33581;
    let t48503 = F::new(192.0) * t33583;
    let t48504 = F::cast_from(0.23392893589820816284e1_f64) * t22653;
    let t48506 = F::cast_from(0.19751789702565206229e-1_f64) * t48472 * t85;
    let t48507 = F::new(144.0) * t22655;
    let t48508 = F::cast_from(0.23392893589820816284e1_f64) * t42535;
    let t48509 = t18941 + t48502 + t48503 - t18950 + t18954 - t48504 + t48506 + t48507 + t18956 + t18885 - t48508 + t18959;
    (t48502, t48503, t48504, t48506, t48507, t48508, t48509)
}
