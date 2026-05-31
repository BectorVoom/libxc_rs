//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1045/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1045<F: Float>(t1874: F, t2048: F, t2041: F, t35: F, t88: F, t22338: F, t85: F, t1872: F, t22610: F, t22700: F, t22703: F, t22705: F, t22708: F, t22711: F, t22713: F, t22716: F, t22719: F) -> (F, F, F, F, F) {
    let t22720 = t2048 * t1874;
    let t22721 = F::cast_from(384.0_f64) * t22720;
    let t22723 = t35 * t2041 * t88;
    let t22724 = F::cast_from(1440.0_f64) * t22723;
    let t22726 = F::cast_from(0.19751789702565206229e-1_f64) * t22338 * t85;
    let t22727 = t2048 * t1872;
    let t22728 = F::cast_from(192.0_f64) * t22727;
    let t22729 = t22700 + t22703 + t22705 + t22708 - t22711 - t22713 - t22716 - t22719 - t22721 + t22724 + t22726 + t22610 - t22728;
    (t22721, t22724, t22726, t22728, t22729)
}
