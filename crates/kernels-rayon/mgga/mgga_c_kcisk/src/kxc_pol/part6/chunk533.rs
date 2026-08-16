//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 533/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk533(t1705: f64, t2404: f64, t2408: f64, t4864: f64, t4881: f64, t1248: f64, t2364: f64, t4889: f64, t2422: f64, t45: f64, t2430: f64, t4928: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7091 = t2404 * t1705;
    let t7099 = t4864 * t2408;
    let t7115 = t4881 * t2408;
    let t7122 = t1248 * t4889 * t2364;
    let t7151 = t45 * t2422;
    let t7156 = t4928 * t2430;
    (t7091, t7099, t7115, t7122, t7151, t7156)
}
