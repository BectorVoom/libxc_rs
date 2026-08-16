//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2535/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2535(t51000: f64, t51004: f64, t51007: f64, t51010: f64, t51012: f64, t51014: f64, t51016: f64, t51018: f64, t51021: f64, t51024: f64, t51027: f64, t51030: f64) -> f64 {
    let t51346 = 0.929655e1_f64 * t51000 + 0.17215833333333333333e1_f64 * t51004 - 0.6618234375e1_f64 * t51007 + 0.2366859375e0_f64 * t51010 - 0.52945875e1_f64 * t51012 - 0.17648625e1_f64 * t51014 + 0.94674375e0_f64 * t51016 + 0.31558125e0_f64 * t51018 + 0.794188125e1_f64 * t51021 - 0.473371875e0_f64 * t51024 - 0.52945875e1_f64 * t51027 + 0.94674375e0_f64 * t51030;
    t51346
}
