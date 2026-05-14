//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 580/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk580<F: Float>(t363: F, t309: F, t5009: F, t372: F, t4993: F, t310: F, t15: F, t741: F, t1468: F, t394: F, t1284: F, t5012: F, t1403: F, t4998: F, t130: F, t4977: F) -> (F, F, F, F, F, F) {
    let t5266 = t363 * t363;
    let t5267 = 1.0 / t5266;
    let t5272 = t5009 * t309;
    let t5273 = t372 * t4993;
    let t5274 = t310 * t5273;
    let t5276 = 0.08230132705969918 * t5272 * t5274;
    let t5279 = t15 * t741;
    let t5285 = t394 * t1468;
    let t5286 = t5285 * t1284;
    let t5288 = 4.4281498357666145 * t5286 * t5012;
    let t5290 = 1.4760499452555382 * t1403 * t4998;
    let t5293 = t130 * t4977;
    (t5267, t5276, t5279, t5288, t5290, t5293)
}
