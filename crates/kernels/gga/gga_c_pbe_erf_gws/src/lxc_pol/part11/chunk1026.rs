//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1026/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1026<F: Float>(t48472: F, t85: F, t22655: F, t42535: F, t18885: F, t18941: F, t18950: F, t18954: F, t18956: F, t18959: F, t48502: F, t48503: F, t48504: F, t33596: F, t33598: F, t33604: F) -> (F, F, F, F, F, F, F) {
    let t48506 = 0.19751789702565206229e-1 * t48472 * t85;
    let t48507 = 144.0 * t22655;
    let t48508 = 0.23392893589820816284e1 * t42535;
    let t48509 = t18941 + t48502 + t48503 - t18950 + t18954 - t48504 + t48506 + t48507 + t18956 + t18885 - t48508 + t18959;
    let t48510 = 0.70178680769462448852e1 * t33596;
    let t48511 = 48.0 * t33598;
    let t48512 = 0.65061485296689145287e-1 * t33604;
    (t48506, t48507, t48508, t48509, t48510, t48511, t48512)
}
