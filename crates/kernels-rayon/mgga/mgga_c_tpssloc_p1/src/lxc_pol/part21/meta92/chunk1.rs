//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 658/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk658(t666: f64, t2331: f64, t614: f64, t94: f64, tau0: f64) -> (f64, f64, f64, f64) {
    let t2332 = t666 * t666;
    let t2333 = t2331 * t2332;
    let t2336 = tau0 * t614;
    let t2341 = 1.0_f64 / t94;
    (t2332, t2333, t2336, t2341)
}
