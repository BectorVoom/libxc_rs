//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1309/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1309(t1437: f64, t21110: f64, t1451: f64, t21073: f64, t1430: f64, t21078: f64, t21130: f64, t542: f64, t21134: f64, t111: f64, t11920: f64, t11951: f64, t11952: f64, t11960: f64, t120: f64, t12070: f64, t17096: f64, t17098: f64, t17100: f64, t17103: f64, t17146: f64, t20882: f64, t20984: f64, t21655: f64, t21665: f64, t4023: f64, t4093: f64, t4865: f64, t4881: f64) -> f64 {
    let t21668 = t1437 * t21110;
    let t21672 = t1451 * t21073;
    let t21675 = t1430 * t21078;
    let t21678 = t542 * t21130;
    let t21681 = t1430 * t21134;
    let t21684 = 0.47822877300252710492e-1_f64 * t17096 - 0.62154466893555682512e-3_f64 * t17098 + 0.47822877300252710492e-1_f64 * t17100 - 0.41436311262370455008e-3_f64 * t17103 + 0.15538616723388920628e-3_f64 * t4093 * t20882 + 0.71734315950379065738e-1_f64 * t11920 * t20984 + 0.95645754600505420984e-1_f64 * t11951 * t21655 - 0.62154466893555682512e-3_f64 * t12070 * t20984 - 0.62154466893555682512e-3_f64 * t17146 * t21655 - 0.23911438650126355246e-1_f64 * t4023 * t20882 - 0.31077233446777841256e-3_f64 * t11952 + 0.4755e-2_f64 * t111 * t21665 + 0.634e-2_f64 * t4865 * t21668 + 0.23911438650126355246e-1_f64 * t11960 - 0.10082625e-4_f64 * t120 * t21672 - 0.672175e-5_f64 * t120 * t21675 + 0.22405833333333333333e-5_f64 * t120 * t21678 + 0.26887e-4_f64 * t4881 * t21681;
    t21684
}
