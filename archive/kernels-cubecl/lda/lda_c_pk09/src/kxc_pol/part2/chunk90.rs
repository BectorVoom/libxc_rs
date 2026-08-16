//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 90/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk90<F: Float>(t44: F, t51: F, t274: F, t275: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t276 = F::powf(t44, t274);
    let t277 = piecewise3::<F>(t45, t275, t276);
    let t278 = F::powf(t51, t274);
    let t279 = piecewise3::<F>(t52, t275, t278);
    let t280 = t277 + t279;
    (t276, t278, t280)
}
