//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 804/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk804<F: Float>(t44: F, t9699: F, t9700: F, t4895: F, t620: F, t2459: F, t1193: F, t1195: F, t1197: F, t2455: F, t2460: F, t4882: F, t9680: F, t9689: F, t9692: F, t9696: F, t1202: F, t2463: F, t276: F, t9683: F, zeta_threshold: F) -> (F, F) {
    let t45 = t44 <= zeta_threshold;
    let t9701 = t9699 * t9700;
    let t9704 = t4895 * t620;
    let t9705 = t2459 * t9704;
    let t9708 = t9680 * t1193 + t2455 * t620 * t1197 + t9689 - t9692 + 1.28 * t4882 * t2460 - 1.28 * t1195 * t9696 + 2.56 * t1195 * t9701 - 1.28 * t1195 * t9705;
    let t9711 = piecewise3(t45, t9683, t1202 * t2463 + t276 * t9708);
    (t9704, t9711)
}
