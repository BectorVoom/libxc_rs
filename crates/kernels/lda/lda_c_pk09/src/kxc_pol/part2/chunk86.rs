//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 86/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk86<F: Float>(t256: F, t258: F, t3: F, t4: F, t254: F, t255: F) -> (F, F, F, F, F, F, F) {
    let t259 = t256 * t258;
    let t260 = F::cast_from(0.580344063812248_f64) * t259;
    let t261 = t3 * t3;
    let t262 = t4 * t4;
    let t263 = F::cast_from(1.0_f64) / t262;
    let t265 = F::cast_from(0.0109912236729144_f64) * t261 * t263;
    let t266 = -F::cast_from(0.32481568604919886_f64) + t254 - t255 + t260 - t265;
    (t259, t260, t261, t262, t263, t265, t266)
}
