//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1414/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1414(t11153: f64, t3584: f64, t1734: f64, t3508: f64, t3548: f64, t4889: f64, t135: f64, t5045: f64, t1174: f64, t1222: f64, t4966: f64, t1215: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15654 = t3584 * t11153;
    let t15659 = t1734 * t3508;
    let t15671 = t4889 * t3548 / 162.0_f64;
    let t15689 = t135 * t5045;
    let t15691 = t1174 * t15689 / 432.0_f64;
    let t15699 = t4966 * t1222 / 2304.0_f64;
    let t15700 = t1734 * t1215;
    (t15654, t15659, t15671, t15691, t15699, t15700)
}
