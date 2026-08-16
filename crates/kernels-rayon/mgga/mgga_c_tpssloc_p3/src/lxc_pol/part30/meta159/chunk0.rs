//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 832/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk832(t4182: f64, t4282: f64, t1510: f64, t2732: f64, t4234: f64, t860: f64, t68: f64, t814: f64, t226: f64) -> (f64, f64, f64, f64, f64) {
    let t4283 = t4282 * t4182;
    let t4286 = t2732 * t1510;
    let t4288 = t860 * t4234;
    let t4290 = t68 * t814;
    let t4291 = t226 * t4290;
    (t4283, t4286, t4288, t4290, t4291)
}
