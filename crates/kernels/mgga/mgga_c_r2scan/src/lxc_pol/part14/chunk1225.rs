//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1225/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1225<F: Float>(t39995: F, t40001: F, t38028: F, t38033: F, t39992: F, t39998: F, t40004: F, t40007: F, t40011: F, t40016: F, t40019: F, t40021: F) -> F {
    let t41649 = F::new(0.27944763721877274748e0) * t39995;
    let t41651 = F::new(0.27944763721877274748e0) * t40001;
    let t41660 = F::new(0.52396431978519890152e-1) * t39992 + t41649 + F::new(0.26198215989259945076e-1) * t39998 + t41651 + F::new(0.26198215989259945076e0) * t40004 - F::new(0.5200933044032561138e0) * t40007 + F::new(0.13099107994629972538e-1) * t40011 + F::new(0.47609969197673950973e-2) * t38028 + F::new(0.62295486109113302474e-1) * t38033 - F::new(0.5200933044032561138e0) * t40016 + F::new(0.86682217400542685632e-1) * t40019 + F::new(0.21951497276451705328e0) * t40021;
    t41660
}
