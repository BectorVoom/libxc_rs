//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1171/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1171<F: Float>(t1430: F, t21078: F, t21130: F, t542: F, t21134: F, t111: F, t11920: F, t11951: F, t11952: F, t11960: F, t120: F, t12070: F, t17096: F, t17098: F, t17100: F, t17103: F, t17146: F, t20882: F, t20984: F, t21655: F, t21665: F, t21668: F, t21672: F, t4023: F, t4093: F, t4865: F, t4881: F) -> (F,) {
    let t21675 = t1430 * t21078;
    let t21678 = t542 * t21130;
    let t21681 = t1430 * t21134;
    let t21684 = 0.47822877300252710492e-1 * t17096 - 0.62154466893555682512e-3 * t17098 + 0.47822877300252710492e-1 * t17100 - 0.41436311262370455008e-3 * t17103 + 0.15538616723388920628e-3 * t4093 * t20882 + 0.71734315950379065738e-1 * t11920 * t20984 + 0.95645754600505420984e-1 * t11951 * t21655 - 0.62154466893555682512e-3 * t12070 * t20984 - 0.62154466893555682512e-3 * t17146 * t21655 - 0.23911438650126355246e-1 * t4023 * t20882 - 0.31077233446777841256e-3 * t11952 + 0.4755e-2 * t111 * t21665 + 0.634e-2 * t4865 * t21668 + 0.23911438650126355246e-1 * t11960 - 0.10082625e-4 * t120 * t21672 - 0.672175e-5 * t120 * t21675 + 0.22405833333333333333e-5 * t120 * t21678 + 0.26887e-4 * t4881 * t21681;
    (t21684,)
}
