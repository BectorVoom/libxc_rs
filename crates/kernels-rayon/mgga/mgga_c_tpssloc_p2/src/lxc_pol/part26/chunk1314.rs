//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1314/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1314(t10109: f64, t225: f64, t10111: f64, t1880: f64, t6553: f64, t23012: f64, t6568: f64, t23270: f64, t25038: f64, t2553: f64, t258: f64, t776: f64) -> (f64, f64, f64) {
    let t82252 = t225 * t10109;
    let t82255 = t1880 * t6553 * t82252 * t10111;
    let t82259 = t23012 * t6568;
    let t82266 = t25038 * t23270 * t258 * t2553 * t776;
    (t82255, t82259, t82266)
}
