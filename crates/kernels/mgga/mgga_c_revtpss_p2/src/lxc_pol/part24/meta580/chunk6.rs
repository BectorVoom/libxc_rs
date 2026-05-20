//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1799/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1799<F: Float>(t12751: F, t12756: F, t1285: F, t1287: F, t17958: F, t1811: F, t21452: F, t21456: F, t21500: F, t24770: F, t24978: F, t24989: F, t24999: F, t3769: F, t3783: F, t45666: F, t45859: F, t45863: F, t6717: F, t6727: F, t72270: F, t72370: F, t89978: F, t90042: F, t90167: F) -> F {
    let t91576 = -F::cast_from(0.79025390195226139184e1_f64) * t21456 * t24999 + F::cast_from(0.15805078039045227836e2_f64) * t45859 * t89978 * t3769 - F::cast_from(0.79025390195226139183e1_f64) * t45863 * t89978 * t3783 - F::cast_from(0.15805078039045227836e2_f64) * t45666 * t90167 * t1287 + F::cast_from(0.15805078039045227836e2_f64) * t21500 * t24978 + F::cast_from(0.15805078039045227836e2_f64) * t21452 * t24978 - F::cast_from(0.79025390195226139183e1_f64) * t72270 * t6717 - F::cast_from(0.79025390195226139183e1_f64) * t12751 * t90042 * t3769 + F::cast_from(0.39512695097613069592e1_f64) * t12756 * t90042 * t3783 - F::cast_from(0.79025390195226139183e1_f64) * t17958 * t24989 + F::cast_from(0.26341796731742046395e1_f64) * t1285 * t1811 * t24770 * t1287 + F::cast_from(0.79025390195226139183e1_f64) * t72370 * t6727;
    t91576
}
