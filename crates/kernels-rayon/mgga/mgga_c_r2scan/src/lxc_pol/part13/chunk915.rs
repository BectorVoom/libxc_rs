//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 915/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk915(t1543: f64, t797: f64, t10610: f64, t3263: f64, t1561: f64, t3347: f64) -> (f64, f64, f64) {
    let t10611 = t797 * t1543;
    let t10613 = t10610 * t3263 * t10611;
    let t10614 = 3.0_f64 / 2.0_f64 * t10613;
    let t10615 = t1561 * t3347;
    (t10611, t10614, t10615)
}
