//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2378/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2378(t2885: f64, t4408: f64, t47705: f64, t47707: f64, t47730: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t47732: f64, t47736: f64, t47738: f64) -> (f64, f64) {
    let t48789 = t4408 * t2885;
    let t48799 = 0.4566222222222222222e-1_f64 * t47705;
    let t48800 = 0.1522074074074074074e-1_f64 * t47707;
    let t48809 = 0.2283111111111111111e-1_f64 * t47730;
    let t48813 = -0.50735802469135802467e-1_f64 * t47681 + 0.20547999999999999999e0_f64 * t47686 - 0.34246666666666666665e-1_f64 * t47691 - 0.34246666666666666665e-1_f64 * t47695 - 0.11415555555555555555e-1_f64 * t47699 - 0.30822e0_f64 * t47703 + t48799 - t48800 + 0.2283111111111111111e-1_f64 * t47709 + 0.11415555555555555555e-1_f64 * t47711 + 0.19025925925925925925e-1_f64 * t47713 - 0.68493333333333333331e-1_f64 * t47715 - 0.34246666666666666665e-1_f64 * t47717 - 0.57077777777777777775e-1_f64 * t47722 - 0.6849333333333333333e-1_f64 * t47724 - 0.41095999999999999999e0_f64 * t47728 - t48809 + 0.17123333333333333333e-1_f64 * t47732 - 0.17123333333333333333e-1_f64 * t47736 + 0.10274e0_f64 * t47738;
    (t48789, t48813)
}
