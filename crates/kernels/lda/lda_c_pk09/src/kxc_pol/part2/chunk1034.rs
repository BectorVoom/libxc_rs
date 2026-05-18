//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1034/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1034<F: Float>(t11191: F, t93: F, t1836: F, t1729: F, t2747: F, t11165: F, t11168: F, t11172: F, t11177: F, t11179: F, t11184: F, t11187: F, t1783: F, t1842: F, t2032: F, t6275: F, t6280: F, t6294: F, t6304: F, t6308: F, t6478: F, t6480: F, t6483: F, t6487: F, t6490: F) -> F {
    let t11192 = t93 * t11191;
    let t11193 = t1836 * t11192;
    let t11195 = t2747 * t1729;
    let t11196 = t93 * t11195;
    let t11201 = F::new(14.216351496367702) * t6275 - F::new(14.216351496367702) * t6280 + F::new(14.216351496367702) * t11165 + F::new(3.5540878740919255) * t1783 * t93 * t11168 + F::new(2.9824072957409817) * t11172 * t2032 - F::new(0.15277772349540736) * t11177 * t11179 + F::new(14.216351496367702) * t1842 * t11184 + F::new(3.5540878740919255) * t1783 * t93 * t11187 - F::new(14.216351496367702) * t11193 + F::new(14.216351496367702) * t1842 * t11196 - t6294 + t6304 - t6308 + F::new(1.6457779058161184) * t6478 - F::new(1.6457779058161184) * t6480 - t6483 + t6487 + t6490;
    t11201
}
