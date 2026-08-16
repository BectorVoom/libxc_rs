//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 804/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk804(t1705: f64, t8692: f64, t4911: f64, t8729: f64, t1248: f64, t4889: f64, t8514: f64, t10999: f64, t8510: f64, t8518: f64, t45: f64, t8740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23496 = t8692 * t1705;
    let t23528 = t8729 * t4911;
    let t23570 = t1248 * t4889 * t8514;
    let t23606 = t1248 * t10999 * t8510;
    let t23609 = t1248 * t4889 * t8518;
    let t23709 = t45 * t8740;
    (t23496, t23528, t23570, t23606, t23609, t23709)
}
