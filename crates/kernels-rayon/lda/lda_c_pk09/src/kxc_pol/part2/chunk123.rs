//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 123/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk123(t372: f64, t94: f64, t333: f64, t294: f64, t305: f64) -> (f64, f64, f64, f64, f64) {
    let t373 = t94 * t372;
    let t374 = t333 * t373;
    let t378 = 1.5625_f64 * t294 + 0.3208669506079574_f64;
    let t381 = f64::atan(0.16004110557090126_f64 / t378);
    let t382 = t381 * t305;
    (t373, t374, t378, t381, t382)
}
