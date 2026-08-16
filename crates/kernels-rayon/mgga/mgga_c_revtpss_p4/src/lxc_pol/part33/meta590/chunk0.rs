//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2005/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2005(t25310: f64, t25331: f64, t2435: f64, t25339: f64, t11064: f64, t7086: f64, t1113: f64, t2411: f64, t530: f64, t7311: f64, t2470: f64, t26049: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93384 = t25310 * t25331;
    let t93391 = t2435 * t25339;
    let t93404 = t7086 * t11064;
    let t94245 = t2411 * t1113;
    let t94345 = t530 * t7311;
    let t94377 = t26049 * t2470;
    (t93384, t93391, t93404, t94245, t94345, t94377)
}
