//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 970/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk970(t23270: f64, t25038: f64, t30622: f64, t5527: f64, t118858: f64, t1880: f64, t214: f64, t225: f64, t258: f64, t28406: f64, t118910: f64, t6552: f64, t7479: f64) -> (f64, f64, f64, f64) {
    let t126398 = 0.9869604401089358619e-1_f64 * t25038 * t23270 * t30622 * t5527;
    let t126399 = 0.76763589786250567036e-1_f64 * t118858;
    let t126404 = 0.16449340668482264365e-1_f64 * t1880 * t214 * t28406 * t225 * t258;
    let t126409 = 0.6579736267392905746e-1_f64 * t6552 * t118910 * t7479;
    (t126398, t126399, t126404, t126409)
}
