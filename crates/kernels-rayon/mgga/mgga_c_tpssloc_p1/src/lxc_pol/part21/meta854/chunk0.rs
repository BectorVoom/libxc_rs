//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3087/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3087(t63953: f64, t63967: f64, t63980: f64, t63994: f64, t1100: f64, t45192: f64, t48140: f64, t55716: f64, t50822: f64, t4756: f64, t3287: f64, t50846: f64, t50848: f64, t50853: f64, t63918: f64, t63921: f64, t63924: f64, t63927: f64, t63930: f64, t63933: f64, t63936: f64, t63939: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t63996 = t63953 + t63967 + t63980 + t63994;
    let t63997 = t1100 * t63996;
    let t64003 = t48140 * t45192 * t55716;
    let t64006 = t48140 * t50822 * t55716;
    let t64008 = t4756 * t4756;
    let t64009 = t3287 * t64008;
    let t64011 = -0.85199506172839506175e-1_f64 * t63918 - 0.54771111111111111112e-1_f64 * t63921 - 0.27385555555555555556e-1_f64 * t63924 - 0.16431333333333333333e0_f64 * t63927 + 0.36514074074074074075e-1_f64 * t63930 + 0.43816888888888888889e0_f64 * t63933 + 0.49293999999999999999e0_f64 * t63936 + 0.197176e1_f64 * t63939 + 0.1898925e1_f64 * t63997 - 0.48685432098765432099e0_f64 * t50846 - 0.10954222222222222222e0_f64 * t50848 + 0.36514074074074074074e0_f64 * t50853 - 0.65725333333333333333e0_f64 * t64003 + 0.197176e1_f64 * t64006 + 0.3071625e0_f64 * t64009;
    (t63996, t63997, t64003, t64006, t64008, t64009, t64011)
}
