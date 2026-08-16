//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 338/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk338(t1263: f64, t1336: f64, t318: f64, t1240: f64, t309: f64, t310: f64) -> (f64, f64, f64) {
    let t1621 = 0.10237773105191754_f64 * t1263;
    let t1623 = t318 * t1336;
    let t1625 = t309 * t310 * t1240;
    (t1621, t1623, t1625)
}
