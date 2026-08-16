//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1011/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1011(t1518: f64, t7683: f64, t1453: f64, t1519: f64, t2322: f64, t27060: f64, t28062: f64, t28065: f64, t28069: f64, t28165: f64, t28170: f64, t28175: f64, t28179: f64, t29427: f64, t29437: f64, t4254: f64, t569: f64, t651: f64, t671: f64, t8158: f64, t8237: f64) -> (f64, f64) {
    let t29444 = t7683 * t1518;
    let t29451 = t1453 * t8237 - 2.0_f64 * t1519 * t27060 - 2.0_f64 * t2322 * t8158 - 2.0_f64 * t29427 * t671 + t29437 * t569 - 2.0_f64 * t29444 * t651 - 2.0_f64 * t4254 * t8158 - t28062 - t28065 - t28069 + t28165 + t28170 + t28175 + t28179;
    (t29444, t29451)
}
