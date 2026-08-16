//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 913/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk913<F: Float>(t4886: F, t9695: F, t2: F, t271: F, t1197: F, t258: F, t4895: F, t620: F, t2459: F, t1193: F, t1195: F, t2455: F, t2460: F, t4882: F, t9680: F, t9689: F, t9692: F) -> (F, F, F, F) {
    let t9696 = t9695 * t4886;
    let t9699 = t271 * t2;
    let t9700 = t258 * t1197;
    let t9701 = t9699 * t9700;
    let t9704 = t4895 * t620;
    let t9705 = t2459 * t9704;
    let t9708 = t9680 * t1193 + t2455 * t620 * t1197 + t9689 - t9692 + F::cast_from(1.28_f64) * t4882 * t2460 - F::cast_from(1.28_f64) * t1195 * t9696 + F::cast_from(2.56_f64) * t1195 * t9701 - F::cast_from(1.28_f64) * t1195 * t9705;
    (t9699, t9700, t9704, t9708)
}
