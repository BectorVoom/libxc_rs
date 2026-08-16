//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3087/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3087<F: Float>(t63953: F, t63967: F, t63980: F, t63994: F, t1100: F, t45192: F, t48140: F, t55716: F, t50822: F, t4756: F, t3287: F, t50846: F, t50848: F, t50853: F, t63918: F, t63921: F, t63924: F, t63927: F, t63930: F, t63933: F, t63936: F, t63939: F) -> (F, F, F, F, F, F, F) {
    let t63996 = t63953 + t63967 + t63980 + t63994;
    let t63997 = t1100 * t63996;
    let t64003 = t48140 * t45192 * t55716;
    let t64006 = t48140 * t50822 * t55716;
    let t64008 = t4756 * t4756;
    let t64009 = t3287 * t64008;
    let t64011 = -F::cast_from(0.85199506172839506175e-1_f64) * t63918 - F::cast_from(0.54771111111111111112e-1_f64) * t63921 - F::cast_from(0.27385555555555555556e-1_f64) * t63924 - F::cast_from(0.16431333333333333333e0_f64) * t63927 + F::cast_from(0.36514074074074074075e-1_f64) * t63930 + F::cast_from(0.43816888888888888889e0_f64) * t63933 + F::cast_from(0.49293999999999999999e0_f64) * t63936 + F::cast_from(0.197176e1_f64) * t63939 + F::cast_from(0.1898925e1_f64) * t63997 - F::cast_from(0.48685432098765432099e0_f64) * t50846 - F::cast_from(0.10954222222222222222e0_f64) * t50848 + F::cast_from(0.36514074074074074074e0_f64) * t50853 - F::cast_from(0.65725333333333333333e0_f64) * t64003 + F::cast_from(0.197176e1_f64) * t64006 + F::cast_from(0.3071625e0_f64) * t64009;
    (t63996, t63997, t64003, t64006, t64008, t64009, t64011)
}
