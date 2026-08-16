//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1971/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1971(t28042: f64, t508: f64, t651: f64, t118: f64, t1519: f64, t25805: f64, t27145: f64, t27152: f64, t27156: f64, t27830: f64, t27834: f64, t27835: f64, t28022: f64, t28025: f64, t28030: f64, t4254: f64, t4257: f64, t4293: f64, t4297: f64, t671: f64, t6985: f64, t7746: f64) -> (f64, f64) {
    let t28043 = t508 * t28042;
    let t28045 = 2.0_f64 * t651 * t28043;
    let t28046 = -t118 * t27830 - 2.0_f64 * t1519 * t25805 - 2.0_f64 * t1519 * t28025 - 2.0_f64 * t27145 * t651 - 2.0_f64 * t28030 * t671 - 2.0_f64 * t4254 * t7746 - 2.0_f64 * t4257 * t6985 - 2.0_f64 * t4293 * t6985 - 2.0_f64 * t4297 * t6985 + t27152 - t27156 + t27834 + t27835 + t28022 - t28045;
    (t28043, t28046)
}
