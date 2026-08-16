//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2146/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2146(t1174: f64, t1709: f64, t44633: f64, t11530: f64, t4889: f64, t50853: f64, t51039: f64, t51051: f64, t457: f64, t4936: f64, t698: f64, t11529: f64, t4912: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52281 = t1174 * t44633 * t1709;
    let t52288 = t4889 * t11530;
    let t52313 = 5.0_f64 / 9.0_f64 * t50853;
    let t52339 = 10.0_f64 / 9.0_f64 * t51039;
    let t52343 = 5.0_f64 / 27.0_f64 * t51051;
    let t52354 = t1174 * t698 * t457 * t4936;
    let t52355 = 0.55555555555555555554e-3_f64 * t52354;
    let t52367 = t1174 * t11529 * t4912;
    (t52281, t52288, t52313, t52339, t52343, t52355, t52367)
}
