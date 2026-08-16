//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1870/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1870(t22986: f64, t25249: f64, t2679: f64, t6646: f64, t23110: f64, t25299: f64, t81651: f64, t23168: f64, t25313: f64, t25319: f64, t2553: f64, t6552: f64, t6637: f64) -> (f64, f64, f64, f64) {
    let t87517 = t22986 * t6646 * t25249 * t2679;
    let t87520 = t81651 * t23110 * t25299;
    let t87522 = t23168 * t25313;
    let t87527 = t6552 * t6637 * t25319 * t2553;
    (t87517, t87520, t87522, t87527)
}
