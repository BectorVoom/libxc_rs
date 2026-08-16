//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2058/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2058(t13783: f64, t1920: f64, t4338: f64, t14192: f64, t6717: f64, t13965: f64, t6755: f64, t25577: f64, t3103: f64, t1933: f64, t23479: f64, t88405: f64) -> (f64, f64, f64, f64, f64) {
    let t88625 = t1920 * t13783 * t4338 / 324.0_f64;
    let t88636 = t6717 * t14192 / 432.0_f64;
    let t88645 = t6755 * t13965;
    let t88648 = t25577 * t3103 / 1152.0_f64;
    let t88689 = 0.20186378047070195428e-3_f64 * t1933 * t88405 * t23479;
    (t88625, t88636, t88645, t88648, t88689)
}
