//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 640/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk640<F: Float>(t5420: F, t68: F, t1287: F, t5039: F, t5161: F, t5045: F, t5190: F, t5208: F, t5212: F, t5068: F, t1490: F, t5081: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5421 = t5420 * t68;
    let t5422 = t5421 * t1287;
    let t5426 = F::new(1.2466946262544771) * t5039;
    let t5430 = F::new(8.333333333333334) * t5161;
    let t5439 = F::new(0.8311297508363181) * t5045;
    let t5440 = F::new(0.6944444444444444) * t5190;
    let t5445 = F::new(6.25) * t5208;
    let t5446 = F::new(6.25) * t5212;
    let t5448 = F::new(0.2770432502787727) * t5068;
    let t5455 = t1490 * t5081;
    (t5422, t5426, t5430, t5439, t5440, t5445, t5446, t5448, t5455)
}
