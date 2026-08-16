//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 854/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk854(t25411: f64, t25413: f64, t1950: f64, t2453: f64, t2458: f64, t25372: f64, t25410: f64, t2411: f64, t7086: f64, t11064: f64, t1962: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25414 = t25411 * t25413;
    let t25422 = t2453 * t1950;
    let t25424 = 0.11565819519348392139e-2_f64 * t25422 * t2458;
    let t25431 = t25372 * t25410;
    let t25432 = t25431 * t25413;
    let t25440 = t7086 * t2411;
    let t25445 = t1962 * t11064;
    let t25759 = t2411 * t33;
    (t25414, t25424, t25431, t25432, t25440, t25445, t25759)
}
