//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 840/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk840(t2305: f64, t569: f64, t3254: f64, t7608: f64, t155: f64, t7693: f64, t143: f64, t1091: f64, t2314: f64, t8069: f64, t890: f64, t8073: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8577 = t2305 * t569;
    let t8585 = t3254 * t7608;
    let t8587 = t155 * t7693;
    let t8589 = t143 * t7693;
    let t8592 = t2314 * t1091;
    let t8595 = t890 * t8069;
    let t8597 = t890 * t8073;
    (t8577, t8585, t8587, t8589, t8592, t8595, t8597)
}
