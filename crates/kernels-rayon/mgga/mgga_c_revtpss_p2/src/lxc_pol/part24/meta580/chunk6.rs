//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1799/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1799(t12751: f64, t12756: f64, t1285: f64, t1287: f64, t17958: f64, t1811: f64, t21452: f64, t21456: f64, t21500: f64, t24770: f64, t24978: f64, t24989: f64, t24999: f64, t3769: f64, t3783: f64, t45666: f64, t45859: f64, t45863: f64, t6717: f64, t6727: f64, t72270: f64, t72370: f64, t89978: f64, t90042: f64, t90167: f64) -> f64 {
    let t91576 = -0.79025390195226139184e1_f64 * t21456 * t24999 + 0.15805078039045227836e2_f64 * t45859 * t89978 * t3769 - 0.79025390195226139183e1_f64 * t45863 * t89978 * t3783 - 0.15805078039045227836e2_f64 * t45666 * t90167 * t1287 + 0.15805078039045227836e2_f64 * t21500 * t24978 + 0.15805078039045227836e2_f64 * t21452 * t24978 - 0.79025390195226139183e1_f64 * t72270 * t6717 - 0.79025390195226139183e1_f64 * t12751 * t90042 * t3769 + 0.39512695097613069592e1_f64 * t12756 * t90042 * t3783 - 0.79025390195226139183e1_f64 * t17958 * t24989 + 0.26341796731742046395e1_f64 * t1285 * t1811 * t24770 * t1287 + 0.79025390195226139183e1_f64 * t72370 * t6727;
    t91576
}
