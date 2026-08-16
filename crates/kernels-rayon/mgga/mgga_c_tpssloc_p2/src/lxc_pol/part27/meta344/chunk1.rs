//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1426/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1426(t12560: f64, t12561: f64, t12562: f64, t12563: f64, t12564: f64, t12565: f64, t9225: f64, t3951: f64, t604: f64, t1406: f64, t2239: f64, t1437: f64, t2241: f64) -> (f64, f64, f64, f64) {
    let t12566 = t12560 - t12561 + t12562 - t12563 + t12564 + t12565 - t9225;
    let t12568 = t3951 * t604;
    let t12571 = t1406 * t2239;
    let t12582 = t1437 * t2241;
    (t12566, t12568, t12571, t12582)
}
