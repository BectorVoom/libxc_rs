//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1891/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1891(t23788: f64, t86797: f64, t16596: f64, t83555: f64, t1081: f64, t4303: f64, t28: f64, t40772: f64, t86717: f64, t25365: f64, t1530: f64, t3231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89928 = t23788 * t86797;
    let t89931 = t83555 * t16596;
    let t89941 = t1081 * t4303;
    let t89953 = t40772 * t28;
    let t89954 = t89953 * t86717;
    let t89972 = t83555 * t25365;
    let t89978 = t3231 * t1530;
    (t89928, t89931, t89941, t89954, t89972, t89978)
}
