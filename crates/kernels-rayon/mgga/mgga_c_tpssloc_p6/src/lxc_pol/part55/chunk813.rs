//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 813/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk813(t1268: f64, t8326: f64, t2006: f64, t225: f64, t567: f64, t214: f64, t1985: f64, t2015: f64, t6906: f64) -> (f64, f64, f64, f64, f64) {
    let t8445 = t1268 * t8326;
    let t8446 = 2.0_f64 * t8445;
    let t8454 = t2006 * t225 * t567;
    let t8455 = t214 * t8454;
    let t8457 = 0.16449340668482264365e-1_f64 * t1985 * t8455;
    let t8458 = t6906 * t2015;
    (t8446, t8454, t8455, t8457, t8458)
}
