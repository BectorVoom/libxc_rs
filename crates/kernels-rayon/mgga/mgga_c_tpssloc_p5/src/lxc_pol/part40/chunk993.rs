//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 993/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk993(t1019: f64, t14206: f64, t1615: f64, t3131: f64, t1022: f64, t360: f64, t883: f64, t13566: f64, t13602: f64, t1573: f64, t2904: f64, t4408: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14207 = t14206 * t1019;
    let t14211 = t1615 * t3131;
    let t14218 = t1615 * t1022;
    let t14219 = t360 * t883;
    let t14245 = 0.23744444444444444444e-1_f64 * t13566;
    let t14246 = 0.11872222222222222222e-1_f64 * t13602;
    let t14263 = t1573 * t2904;
    let t14266 = t4408 * t923;
    (t14207, t14211, t14218, t14219, t14245, t14246, t14263, t14266)
}
