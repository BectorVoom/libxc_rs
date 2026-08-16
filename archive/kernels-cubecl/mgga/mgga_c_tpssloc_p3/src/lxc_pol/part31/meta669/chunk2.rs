//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1979/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1979<F: Float>(t101499: F, t16673: F, t226: F, t235: F, t2617: F, t26661: F, t29000: F, t29041: F, t4234: F, t5585: F, t7102: F, t808: F, t812: F, t81600: F, t84851: F, t84962: F, t87119: F, t87127: F, t87140: F, t98416: F, t98420: F, t98425: F, t98428: F, t98432: F, t98435: F) -> F {
    let t101656 = -t87119 + t808 * t29041 + F::cast_from(2.0_f64) * t2617 * t29000 + F::cast_from(2.0_f64) * t812 * t84962 * t5585 - t84851 + F::cast_from(0.52089578783527170489e-1_f64) * t81600 + t87127 + F::cast_from(0.15352717957250113407e0_f64) * t98416 - F::cast_from(2.0_f64) * t812 * t26661 * t4234 - t16673 * t7102 + F::cast_from(0.6579736267392905746e-1_f64) * t87140 + t226 * t235 * t101499 - F::cast_from(0.15352717957250113407e0_f64) * t98420 + F::cast_from(0.3289868133696452873e-1_f64) * t98425 - F::cast_from(0.3289868133696452873e-1_f64) * t98428 + F::cast_from(0.3289868133696452873e-1_f64) * t98432 - F::cast_from(0.16449340668482264365e-1_f64) * t98435;
    t101656
}
