//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1042/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1042<F: Float>(t10524: F, t117: F, t84: F, t1338: F, t1347: F, t2813: F, t415: F, t118: F, t3993: F, t2791: F, t391: F, t1329: F) -> (F, F, F, F, F, F) {
    let t10852 = F::new(0.031505407223141116) * t84 * t10524 * t117;
    let t10853 = t1338 * t1347;
    let t10855 = t2813 * t415;
    let t10857 = t3993 * t118;
    let t10860 = F::new(0.12602162889256446) * t391 * t2791;
    let t10861 = t1329 * t1347;
    (t10852, t10853, t10855, t10857, t10860, t10861)
}
