//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1174/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1174(t110502: f64, t25375: f64, t28314: f64, t99463: f64, t27213: f64, t28360: f64, t28368: f64, t99404: f64, t98849: f64, t30405: f64, t689: f64, t25431: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110503 = t25375 * t110502;
    let t110505 = t99463 * t28314;
    let t110517 = t27213 * t28360;
    let t110525 = t99404 * t28368;
    let t110527 = t98849 * t28368;
    let t110541 = t30405 * t689;
    let t110542 = t25431 * t110541;
    (t110503, t110505, t110517, t110525, t110527, t110541, t110542)
}
