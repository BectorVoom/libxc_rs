//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1093/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1093(t1872: f64, t2758: f64, t11101: f64, t1827: f64, t1800: f64, t507: f64, t1943: f64, t11248: f64, t1856: f64, t11469: f64, t1842: f64, t1672: f64, t2940: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12014 = t1872 * t2758;
    let t12017 = t1827 * t11101;
    let t12018 = t12017 * t1800;
    let t12020 = t507 * t11101;
    let t12023 = t1943 * t2758;
    let t12026 = t1856 * t11248;
    let t12028 = t1842 * t11469;
    let t12030 = t2940 * t1672;
    (t12014, t12018, t12020, t12023, t12026, t12028, t12030)
}
