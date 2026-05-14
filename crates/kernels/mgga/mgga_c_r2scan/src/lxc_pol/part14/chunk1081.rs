//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1081/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1081<F: Float>(t39958: F, t39962: F, t39964: F, t39967: F, t39969: F, t39977: F, t39979: F, t39984: F, t39972: F, t39975: F, t39982: F, t39987: F, t39995: F, t40001: F, t38028: F, t38033: F, t39992: F, t39998: F, t40004: F, t40007: F, t40011: F, t40016: F, t40019: F, t40021: F) -> (F, F) {
    let t41634 = 0.18629842481251516498e0 * t39958;
    let t41635 = 0.28565981518604370584e-1 * t39962;
    let t41636 = 0.95219938395347901946e-2 * t39964;
    let t41637 = 0.95219938395347901946e-2 * t39967;
    let t41638 = 0.28565981518604370584e-1 * t39969;
    let t41641 = 0.42683466926433871473e0 * t39977;
    let t41642 = 0.21951497276451705328e-1 * t39979;
    let t41644 = 0.10975748638225852664e-1 * t39984;
    let t41646 = t41634 + t41635 + t41636 + t41637 + t41638 + 0.5200933044032561138e0 * t39972 + 0.20803732176130244552e1 * t39975 - t41641 - t41642 - 0.92461031893912198007e0 * t39982 + t41644 + 0.26198215989259945076e-1 * t39987;
    let t41649 = 0.27944763721877274748e0 * t39995;
    let t41651 = 0.27944763721877274748e0 * t40001;
    let t41660 = 0.52396431978519890152e-1 * t39992 + t41649 + 0.26198215989259945076e-1 * t39998 + t41651 + 0.26198215989259945076e0 * t40004 - 0.5200933044032561138e0 * t40007 + 0.13099107994629972538e-1 * t40011 + 0.47609969197673950973e-2 * t38028 + 0.62295486109113302474e-1 * t38033 - 0.5200933044032561138e0 * t40016 + 0.86682217400542685632e-1 * t40019 + 0.21951497276451705328e0 * t40021;
    (t41646, t41660)
}
