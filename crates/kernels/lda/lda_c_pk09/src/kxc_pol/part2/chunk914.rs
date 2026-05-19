//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 914/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk914<F: Float>(t44: F, t1202: F, t2463: F, t276: F, t9683: F, t9708: F, t2468: F, t4875: F, t2467: F, t4910: F, t4821: F, t1179: F, t2146: F, zeta_threshold: F) -> (F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t9711 = piecewise3::<F>(t45, t9683, t1202 * t2463 + t276 * t9708);
    let t9717 = F::new(1.28) * t4875 * t2468;
    let t9718 = t2467 * t4910;
    let t9720 = F::new(1.28) * t4821 * t9718;
    let t9723 = t1179 * t2146;
    (t9711, t9717, t9720, t9723)
}
