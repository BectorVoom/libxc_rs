//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2535/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2535<F: Float>(t51000: F, t51004: F, t51007: F, t51010: F, t51012: F, t51014: F, t51016: F, t51018: F, t51021: F, t51024: F, t51027: F, t51030: F) -> F {
    let t51346 = F::cast_from(0.929655e1_f64) * t51000 + F::cast_from(0.17215833333333333333e1_f64) * t51004 - F::cast_from(0.6618234375e1_f64) * t51007 + F::cast_from(0.2366859375e0_f64) * t51010 - F::cast_from(0.52945875e1_f64) * t51012 - F::cast_from(0.17648625e1_f64) * t51014 + F::cast_from(0.94674375e0_f64) * t51016 + F::cast_from(0.31558125e0_f64) * t51018 + F::cast_from(0.794188125e1_f64) * t51021 - F::cast_from(0.473371875e0_f64) * t51024 - F::cast_from(0.52945875e1_f64) * t51027 + F::cast_from(0.94674375e0_f64) * t51030;
    t51346
}
