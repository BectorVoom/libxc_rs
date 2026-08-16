//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 365/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk365(t1754: f64, t1765: f64, t1684: f64, t1735: f64, t1732: f64, t1738: f64, t1762: f64, t1769: f64) -> (f64, f64, f64, f64, f64) {
    let t1784 = 2.0_f64 * t1754;
    let t1786 = 0.6666666666666666_f64 * t1765;
    let t1788 = 0.505765839233979_f64 * t1684;
    let t1790 = 0.168588613077993_f64 * t1735;
    let t1792 = t1784 - 2.0_f64 * t1762 + t1786 + 2.0_f64 * t1769 + t1788 - 0.505765839233979_f64 * t1732 + t1790 + 0.505765839233979_f64 * t1738;
    (t1784, t1786, t1788, t1790, t1792)
}
