//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 475/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk475(t2558: f64, t2610: f64, t1409: f64, t1411: f64, t2502: f64, t2505: f64, t1389: f64, t1391: f64, t1393: f64, t1395: f64, t2542: f64, t2546: f64) -> (f64, f64, f64) {
    let t2611 = t2558 + t2610;
    let t2615 = t1409 - 0.9421211958699838_f64 * t2502 + t1411 + 0.9421211958699838_f64 * t2505;
    let t2621 = t1389 - 2.0_f64 * t2542 + t1391 + 2.0_f64 * t2546 + t1393 - 0.505765839233979_f64 * t2502 + t1395 + 0.505765839233979_f64 * t2505;
    (t2611, t2615, t2621)
}
