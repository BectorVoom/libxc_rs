//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 979/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk979(t10454: f64, t1625: f64, t10020: f64, t1285: f64, t2665: f64, t5239: f64, t306: f64, t1277: f64, t309: f64, t310: f64, t1382: f64, t2487: f64) -> (f64, f64, f64, f64, f64) {
    let t10455 = t10454 * t1625;
    let t10459 = t1285 * t10020;
    let t10465 = t2665 * t5239;
    let t10466 = t10465 * t306;
    let t10468 = t309 * t310 * t1277;
    let t10471 = t1382 * t2487;
    (t10455, t10459, t10466, t10468, t10471)
}
