//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 302/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk302(t1240: f64, t372: f64, t310: f64, t1337: f64, t1284: f64, t355: f64) -> (f64, f64, f64, f64) {
    let t1338 = t372 * t1240;
    let t1339 = t310 * t1338;
    let t1341 = 0.04115066352984959_f64 * t1337 * t1339;
    let t1342 = t355 * t1284;
    (t1338, t1339, t1341, t1342)
}
