//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 405/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk405(t2016: f64, t444: f64, t305: f64, t461: f64, t1947: f64, t451: f64, t1240: f64, t309: f64, t454: f64) -> (f64, f64, f64, f64, f64) {
    let t2035 = t2016 * t444;
    let t2036 = t461 * t305;
    let t2037 = t2035 * t2036;
    let t2040 = t451 * t1947;
    let t2042 = t309 * t454 * t1240;
    (t2035, t2036, t2037, t2040, t2042)
}
