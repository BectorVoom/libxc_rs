//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1944/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1944(t1361: f64, t16153: f64, t26288: f64, t1339: f64, t16206: f64, t6936: f64, t1825: f64, t22827: f64, t3719: f64, t1307: f64, t7708: f64, t80840: f64, t90787: f64) -> (f64, f64, f64, f64) {
    let t91333 = t26288 * t1361 * t16153;
    let t91336 = t6936 * t1339 * t16206;
    let t91340 = t22827 * t1339 * t1825 * t3719;
    let t91344 = t80840 * t90787 * t7708 * t1307;
    (t91333, t91336, t91340, t91344)
}
