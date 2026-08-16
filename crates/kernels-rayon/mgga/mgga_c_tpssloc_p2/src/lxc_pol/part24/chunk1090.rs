//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1090/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1090(t1022: f64, t607: f64, t1307: f64, t1388: f64, t1351: f64, t2319: f64, t576: f64, t671: f64, t1874: f64, t9348: f64, t111: f64, t6514: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14227 = t607 * t1022;
    let t15904 = t1388 * t1307;
    let t16312 = t1351 * t1307;
    let t16535 = t576 * t2319;
    let t20173 = t576 * t671;
    let t22460 = 2.0_f64 * t9348 * t1874;
    let t22461 = t6514 * t111;
    (t14227, t15904, t16312, t16535, t20173, t22460, t22461)
}
