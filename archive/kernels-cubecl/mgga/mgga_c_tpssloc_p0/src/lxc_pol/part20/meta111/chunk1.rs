//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 742/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk742<F: Float>(t193: F, t202: F, t2486: F, t2522: F, t2523: F, t2530: F, t2533: F, t2537: F, t2539: F, t2553: F, t2654: F, t2657: F, t2661: F, t2665: F, t2745: F, t2749: F, t2752: F, t766: F, t776: F, t870: F) -> F {
    let t2755 = t193 * t202 * t2745 * t870 - t193 * t202 * t2749 * t2752 + F::cast_from(3.0_f64) * t193 * t2553 * t766 + F::cast_from(6.0_f64) * t2522 * t2523 * t776 - t2486 - t2530 - t2533 - t2537 + t2539 - t2654 + t2657 + t2661 + t2665;
    t2755
}
