//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 668/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk668<F: Float>(t2291: F, t748: F, t155: F, t3121: F, t3123: F, t3131: F, t3132: F, t3149: F, t3165: F, t3173: F, t3177: F, t3191: F, t7691: F, t7694: F, t7706: F, t119: F, t2238: F) -> (F, F) {
    let t7709 = t748 * t2291;
    let t7714 = -14.71989892086604 * t3121 - 14.71989892086604 * t3123 - t3131 + 0.027433775686566395 * t3132 - 1.8805371096875316 * t7691 - 1.2536914064583544 * t7694 + 2.9824072957409817 * t3149 + t3165 - 19.489173774580152 * t155 * t7706 + 0.027433775686566395 * t7709 - 3.600163427964126 * t3173 + 3.600163427964126 * t3177 - 3.600163427964126 * t3191;
    let t7727 = t2238 * t119;
    (t7714, t7727)
}
