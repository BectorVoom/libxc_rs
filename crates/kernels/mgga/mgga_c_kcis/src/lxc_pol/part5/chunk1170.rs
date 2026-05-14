//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1170/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1170<F: Float>(t1445: F, t7113: F, t532: F, t7119: F, t1401: F, t7123: F, t7142: F, t1419: F, t21624: F, t1650: F, t167: F, t1437: F, t21106: F, t21110: F, t1451: F, t21073: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21631 = t1445 * t7113;
    let t21633 = t532 * t7119;
    let t21635 = t1401 * t7123;
    let t21637 = t1401 * t7142;
    let t21641 = t21624 * t1419;
    let t21655 = t1650 * t167;
    let t21665 = t1437 * t21106;
    let t21668 = t1437 * t21110;
    let t21672 = t1451 * t21073;
    (t21631, t21633, t21635, t21637, t21641, t21655, t21665, t21668, t21672)
}
