//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2050/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2050(t1444: f64, t543: f64, t268: f64, t4102: f64, t94395: f64, t4057: f64, t676: f64, t25880: f64, t25904: f64, t25945: f64, t9285: f64, t25944: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94396 = t543 * t1444;
    let t94398 = t268 * t4102 * t94396;
    let t94399 = t94395 * t94398;
    let t94403 = t676 * t4057;
    let t94404 = t25880 * t94403;
    let t94405 = t25904 * t94404;
    let t94407 = t25945 * t9285;
    let t94409 = 0.68540937416128198417e-2_f64 * t25944 * t94407;
    (t94398, t94399, t94404, t94405, t94407, t94409)
}
