//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 596/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk596<F: Float>(t823: F, t825: F, t609: F, t121: F, t4037: F, t340: F, t89: F, t3141: F, t1448: F, t337: F, t280: F, t1445: F) -> (F, F, F, F, F, F) {
    let t4710 = t823 * t825;
    let t4711 = t4710 * t609;
    let t4712 = t121 * t4711;
    let t4713 = t4037 * t4712;
    let t4715 = t89 * t340;
    let t4725 = t89 * t3141;
    let t4753 = t1448 * t337;
    let t4754 = t4753 * t280;
    let t4755 = t1445 * t4754;
    (t4710, t4713, t4715, t4725, t4754, t4755)
}
