//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1206/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1206(t11883: f64, t1215: f64, t6252: f64, t1751: f64, t5011: f64, t1246: f64, t6238: f64, t19145: f64, t3612: f64, t1734: f64, t5052: f64, t1235: f64, t6218: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19165 = t11883 * t1215;
    let t19166 = t6252 * t19165;
    let t19169 = t1751 * t5011;
    let t19170 = t19169 * t1246;
    let t19173 = t6238 * t1215;
    let t19174 = t19173 * t1246;
    let t19176 = t19145 * t3612;
    let t19179 = t5052 * t1734;
    let t19180 = t19179 * t1246;
    let t19189 = t1235 * t6218;
    (t19166, t19170, t19174, t19176, t19180, t19189)
}
