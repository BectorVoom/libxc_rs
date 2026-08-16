//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1287/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1287(t11322: f64, t1889: f64, t3815: f64, t1897: f64, t3781: f64, t1319: f64, t5481: f64, t3809: f64, t1958: f64, t3820: f64, t1317: f64, t5523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16480 = t11322 * t1889 * t3815;
    let t16483 = t1897 * t3781;
    let t16488 = t5481 * t1319;
    let t16491 = t1897 * t3809;
    let t16500 = t3820 * t1958;
    let t16503 = t1317 * t5523;
    (t16480, t16483, t16488, t16491, t16500, t16503)
}
