//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 994/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk994(t1504: f64, t2594: f64, t10020: f64, t1319: f64, t2507: f64, t5267: f64, t306: f64, t1329: f64, t309: f64, t310: f64, t2648: f64, t5248: f64) -> (f64, f64, f64, f64, f64) {
    let t10703 = t2594 * t1504;
    let t10706 = t1319 * t10020;
    let t10712 = t2507 * t5267;
    let t10713 = t10712 * t306;
    let t10715 = t309 * t310 * t1329;
    let t10719 = t2648 * t5248;
    (t10703, t10706, t10713, t10715, t10719)
}
