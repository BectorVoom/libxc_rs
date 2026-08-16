//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2588/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2588(t11560: f64, t4889: f64, t1174: f64, t1716: f64, t2402: f64, t4930: f64, t698: f64, t11513: f64, t11589: f64, t15313: f64, t3447: f64, t14749: f64, t15402: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52076 = t4889 * t11560;
    let t52081 = t1174 * t2402 * t1716;
    let t52084 = t1174 * t698 * t4930;
    let t52086 = t4889 * t11513;
    let t52089 = t3447 * t11589 * t15313;
    let t52092 = t3447 * t15402 * t14749;
    (t52076, t52081, t52084, t52086, t52089, t52092)
}
