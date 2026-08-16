//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 959/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk959(t9959: f64, t9966: f64, t2345: f64, t4438: f64, t177: f64, t4377: f64, t737: f64, t10022: f64, t10120: f64, t774: f64, t1232: f64, t1625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12754 = 12.0_f64 * t9959;
    let t12757 = 80.0_f64 * t9966;
    let t12758 = t4438 * t2345;
    let t12767 = t4377 * t177;
    let t12769 = 0.11696447245269292414e1_f64 * t12767 * t737;
    let t12780 = 48.0_f64 * t10022;
    let t12816 = t10120 * t774;
    let t12817 = t1625 * t1232;
    (t12754, t12757, t12758, t12769, t12780, t12816, t12817)
}
