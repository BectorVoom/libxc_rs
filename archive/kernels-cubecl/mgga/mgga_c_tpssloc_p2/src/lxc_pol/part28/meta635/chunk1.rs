//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2012/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2012<F: Float>(t90707: F, t90749: F, t90759: F, t90781: F, t90789: F, t90791: F, t90794: F, t90797: F, t12240: F, t16033: F, t27074: F, t27078: F, t5334: F, t90747: F, t90752: F, t90757: F, t90763: F, t90770: F, t90774: F, t90778: F, t90785: F) -> (F, F) {
    let t93467 = F::cast_from(0.76763589786250567036e-1_f64) * t90707;
    let t93473 = F::cast_from(0.15352717957250113407e0_f64) * t90749;
    let t93476 = F::cast_from(0.76763589786250567036e-1_f64) * t90759;
    let t93483 = F::cast_from(0.16449340668482264365e-1_f64) * t90781;
    let t93488 = F::cast_from(0.9869604401089358619e-1_f64) * t90789;
    let t93489 = F::cast_from(0.15352717957250113407e0_f64) * t90791;
    let t93490 = F::cast_from(0.3289868133696452873e-1_f64) * t90794;
    let t93491 = F::cast_from(0.3289868133696452873e-1_f64) * t90797;
    let t93492 = F::cast_from(0.3289868133696452873e-1_f64) * t90747 - t93473 - F::cast_from(0.16449340668482264365e-1_f64) * t90752 + F::cast_from(0.19739208802178717238e0_f64) * t90757 + t93476 - F::cast_from(0.9869604401089358619e-1_f64) * t90763 - F::cast_from(0.3289868133696452873e-1_f64) * t90770 - F::cast_from(2.0_f64) * t16033 * t27078 + F::cast_from(0.6579736267392905746e-1_f64) * t90774 + F::cast_from(0.3289868133696452873e-1_f64) * t90778 + t93483 + F::cast_from(2.0_f64) * t5334 * t27074 * t12240 - F::cast_from(0.16449340668482264365e-1_f64) * t90785 - t93488 + t93489 + t93490 + t93491;
    (t93467, t93492)
}
