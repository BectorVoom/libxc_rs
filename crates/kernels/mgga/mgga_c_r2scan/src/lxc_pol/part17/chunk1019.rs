//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1019/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1019<F: Float>(t11697: F, t11700: F, t11728: F, t11730: F, t12477: F, t12480: F, t12482: F, t12487: F, t12490: F, t12493: F, t12496: F, t12499: F, t12501: F, t12504: F, t12507: F) -> F {
    let t12782 = F::new(0.95219938395347901946e-2) * t11697 + F::new(0.28565981518604370584e-1) * t11700 + F::new(0.17336443480108537126e0) * t12477 + F::new(0.5200933044032561138e0) * t12480 + F::new(0.21951497276451705328e0) * t12482 + F::new(0.13869154784086829701e1) * t11728 + F::new(0.51220160311720645767e0) * t11730 + F::new(0.17336443480108537126e0) * t12487 + F::new(0.10401866088065122276e1) * t12490 - F::new(0.87327386630866483588e-2) * t12493 - F::new(0.26198215989259945076e-1) * t12496 - F::new(0.86682217400542685632e-1) * t12499 - F::new(0.5200933044032561138e0) * t12501 - F::new(0.2600466522016280569e0) * t12504 + F::new(0.10975748638225852664e0) * t12507;
    t12782
}
