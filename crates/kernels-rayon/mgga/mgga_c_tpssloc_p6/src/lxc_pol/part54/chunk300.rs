//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 300/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk300(t1245: f64, t1246: f64, t1235: f64, t493: f64, t1201: f64, t1244: f64, t470: f64, t494: f64) -> (f64, f64, f64) {
    let t1247 = t1245 * t1246;
    let t1249 = t493 * t1235;
    let t1251 = t1201 * t494 + t1244 * t1247 + t1249 * t470;
    (t1247, t1249, t1251)
}
