//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2161/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2161(t5303: f64, t91100: f64, t1339: f64, t550: f64, t56812: f64, t6936: f64, t12289: f64, t1351: f64, t57342: f64, t20473: f64, t3788: f64, t19930: f64, t6952: f64) -> (f64, f64, f64, f64, f64) {
    let t97322 = t91100 * t5303;
    let t97326 = t6936 * t1339 * t56812 * t550;
    let t97333 = t6936 * t12289 * t57342 * t1351;
    let t97337 = t6936 * t3788 * t20473 * t1351;
    let t97340 = t6952 * t19930;
    (t97322, t97326, t97333, t97337, t97340)
}
