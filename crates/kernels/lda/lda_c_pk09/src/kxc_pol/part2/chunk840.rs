//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 840/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk840<F: Float>(t2305: F, t569: F, t3254: F, t7608: F, t155: F, t7693: F, t143: F, t1091: F, t2314: F, t8069: F, t890: F, t8073: F) -> (F, F, F, F, F, F, F) {
    let t8577 = t2305 * t569;
    let t8585 = t3254 * t7608;
    let t8587 = t155 * t7693;
    let t8589 = t143 * t7693;
    let t8592 = t2314 * t1091;
    let t8595 = t890 * t8069;
    let t8597 = t890 * t8073;
    (t8577, t8585, t8587, t8589, t8592, t8595, t8597)
}
