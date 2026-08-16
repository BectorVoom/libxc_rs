//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 375/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk375(t1800: f64, t1828: f64, t337: f64, t467: f64, t1747: f64) -> (f64, f64, f64) {
    let t1830 = 1.8805371096875316_f64 * t1828 * t1800;
    let t1831 = t467 * t337;
    let t1832 = t1831 * t1747;
    (t1830, t1831, t1832)
}
