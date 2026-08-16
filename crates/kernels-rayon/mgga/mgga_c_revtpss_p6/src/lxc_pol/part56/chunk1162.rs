//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1162/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1162(t28173: f64, t8764: f64, t27060: f64, t7742: f64, t29432: f64, t28063: f64, t7586: f64, t651: f64, t7002: f64, t8233: f64, t28182: f64, t34446: f64, t7003: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t129440 = t8764 * t28173;
    let t129445 = t27060 * t7742;
    let t129447 = t29432 * t7742;
    let t129449 = t7586 * t28063;
    let t129452 = t651 * t8233 * t7002;
    let t129455 = t8764 * t28182;
    let t129457 = t34446 * t7003;
    (t129440, t129445, t129447, t129449, t129452, t129455, t129457)
}
