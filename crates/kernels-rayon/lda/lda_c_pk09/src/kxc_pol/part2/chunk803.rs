//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 803/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk803(t2319: f64, t748: f64, t179: f64, t7693: f64, t161: f64, t3507: f64, t3512: f64, t3514: f64, t3527: f64, t3529: f64, t3534: f64, t3536: f64, t3538: f64, t3555: f64, t3559: f64) -> f64 {
    let t8078 = t748 * t2319;
    let t8080 = t179 * t7693;
    let t8082 = t161 * t7693;
    let t8088 = 0.027433775686566395_f64 * t8078 - 12.423505345088643_f64 * t8080 - 3.2915558116322368_f64 * t8082 + 3.159189221415045_f64 * t3507 - t3512 - t3514 - t3527 - t3529 + 12.992782516386768_f64 * t3534 + 12.992782516386768_f64 * t3536 - 12.992782516386768_f64 * t3538 - t3555 - t3559;
    t8088
}
