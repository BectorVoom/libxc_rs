//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2018/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2018<F: Float>(t5230: F, t7934: F, t90980: F, t93588: F, t93589: F, t93590: F, t93592: F, t93599: F, t93600: F, t97079: F, t97083: F, t97087: F, t97091: F, t97095: F, t97106: F, t97108: F, t97111: F, t97114: F) -> F {
    let t102629 = F::cast_from(2.0_f64) * t5230 * t7934 + t93588 - t93589 - t93590 - F::cast_from(0.16449340668482264365e-1_f64) * t97079 + F::cast_from(0.6579736267392905746e-1_f64) * t97083 + F::cast_from(0.6579736267392905746e-1_f64) * t97087 + F::cast_from(0.6579736267392905746e-1_f64) * t97091 + t93592 + F::cast_from(0.15352717957250113407e0_f64) * t97095 + F::cast_from(0.3289868133696452873e-1_f64) * t90980 + t93599 - t93600 + F::cast_from(0.6579736267392905746e-1_f64) * t97106 + F::cast_from(0.76763589786250567037e-1_f64) * t97108 - F::cast_from(0.82246703342411321825e-2_f64) * t97111 - F::cast_from(0.16449340668482264365e-1_f64) * t97114;
    t102629
}
