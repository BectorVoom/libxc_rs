//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1157/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1157(t27955: f64, t5273: f64, t7617: f64, t5291: f64, t7616: f64, t1241: f64, t5265: f64, t7618: f64, t1219: f64, t8172: f64, t5357: f64, t7607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28885 = 7.0_f64 / 72.0_f64 * t27955;
    let t29010 = t5273 * t7617;
    let t29019 = t7616 * t5291;
    let t29020 = t1241 * t29019;
    let t29023 = t7618 * t5265;
    let t29027 = t8172 * t1219;
    let t29031 = t7607 * t5357;
    (t28885, t29010, t29019, t29020, t29023, t29027, t29031)
}
