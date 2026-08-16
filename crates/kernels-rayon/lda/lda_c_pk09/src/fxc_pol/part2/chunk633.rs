//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 633/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk633(t310: f64, t5273: f64, t5272: f64, t15: f64, t741: f64, t1468: f64, t394: f64, t1284: f64, t5012: f64, t1403: f64, t4998: f64, t130: f64, t4977: f64) -> (f64, f64, f64, f64, f64) {
    let t5274 = t310 * t5273;
    let t5276 = 0.08230132705969918_f64 * t5272 * t5274;
    let t5279 = t15 * t741;
    let t5285 = t394 * t1468;
    let t5286 = t5285 * t1284;
    let t5288 = 4.4281498357666145_f64 * t5286 * t5012;
    let t5290 = 1.4760499452555382_f64 * t1403 * t4998;
    let t5293 = t130 * t4977;
    (t5276, t5279, t5288, t5290, t5293)
}
