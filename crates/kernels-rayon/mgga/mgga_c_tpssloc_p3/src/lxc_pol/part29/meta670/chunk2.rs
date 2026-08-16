//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2242/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2242(t1358: f64, t26248: f64, t3862: f64, t7715: f64, t22705: f64, t22852: f64, t236: f64, t5286: f64, t550: f64, t26245: f64, t80791: f64, t80867: f64) -> (f64, f64, f64, f64, f64) {
    let t91303 = t26248 * t1358;
    let t91304 = 7.0_f64 / 1152.0_f64 * t91303;
    let t91305 = t7715 * t3862;
    let t91310 = t22852 * t22705 * t236 * t5286 * t550;
    let t91311 = 0.6728792682356731809e-4_f64 * t91310;
    let t91312 = t80791 * t26245;
    let t91314 = 119.0_f64 / 864.0_f64 * t80867;
    (t91304, t91305, t91311, t91312, t91314)
}
