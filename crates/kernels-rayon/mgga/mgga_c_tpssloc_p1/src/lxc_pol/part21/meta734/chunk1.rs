//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2593/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2593(t1174: f64, t457: f64, t4936: f64, t698: f64, t15277: f64, t3431: f64, t15281: f64, t15303: f64, t11540: f64, t4889: f64, t11529: f64, t4912: f64) -> (f64, f64, f64, f64, f64) {
    let t52354 = t1174 * t698 * t457 * t4936;
    let t52357 = t1174 * t3431 * t15277;
    let t52362 = t1174 * t15281 * t15303;
    let t52364 = t4889 * t11540;
    let t52367 = t1174 * t11529 * t4912;
    (t52354, t52357, t52362, t52364, t52367)
}
