//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 636/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk636(t8343: f64, t8344: f64, t1894: f64, t1902: f64, t214: f64, t1880: f64, t1268: f64, t8326: f64, t2006: f64, t225: f64, t567: f64, t1985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8345 = t8343 * t8344;
    let t8356 = t1894 * t1902;
    let t8357 = t214 * t8356;
    let t8359 = 0.16449340668482264365e-1_f64 * t1880 * t8357;
    let t8445 = t1268 * t8326;
    let t8446 = 2.0_f64 * t8445;
    let t8454 = t2006 * t225 * t567;
    let t8455 = t214 * t8454;
    let t8457 = 0.16449340668482264365e-1_f64 * t1985 * t8455;
    (t8345, t8356, t8357, t8359, t8446, t8454, t8455, t8457)
}
