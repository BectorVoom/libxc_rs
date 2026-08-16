//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 233/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk233(t1032: f64, t342: f64, t358: f64, t360: f64, t336: f64, t368: f64) -> (f64, f64, f64, f64, f64) {
    let t1033 = t342 * t1032;
    let t1034 = t358 * t358;
    let t1035 = 1.0_f64 / t1034;
    let t1036 = t1035 * t360;
    let t1038 = 1.0_f64 / t368 / t336;
    (t1033, t1034, t1035, t1036, t1038)
}
