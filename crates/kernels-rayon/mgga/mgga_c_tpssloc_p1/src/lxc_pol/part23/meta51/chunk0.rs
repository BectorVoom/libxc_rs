//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 326/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk326(t500: f64, t111: f64, t88: f64, t522: f64, t588: f64, t592: f64, t521: f64, t750: f64) -> (f64, f64, f64, f64, f64) {
    let t1256 = 1.0_f64 / t500;
    let t1268 = t88 * t111;
    let t1274 = 4.0_f64 * t588 * t522;
    let t1276 = 4.0_f64 * t592 * t522;
    let t1287 = t521 * t750;
    (t1256, t1268, t1274, t1276, t1287)
}
