//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 86/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk86(t256: f64, t258: f64, t3: f64, t4: f64, t254: f64, t255: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t259 = t256 * t258;
    let t260 = 0.580344063812248_f64 * t259;
    let t261 = t3 * t3;
    let t262 = t4 * t4;
    let t263 = 1.0_f64 / t262;
    let t265 = 0.0109912236729144_f64 * t261 * t263;
    let t266 = -0.32481568604919886_f64 + t254 - t255 + t260 - t265;
    (t259, t260, t261, t262, t263, t265, t266)
}
