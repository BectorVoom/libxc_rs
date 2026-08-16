//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 951/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk951(t3868: f64, t4150: f64, t118: f64, t1310: f64, t1315: f64, t1453: f64, t2320: f64, t2322: f64, t2328: f64, t2331: f64, t2372: f64, t3813: f64, t3821: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64) -> (f64, f64) {
    let t4151 = t3868 + t4150;
    let t4153 = -t118 * t3813 - 2.0_f64 * t1310 * t649 + 2.0_f64 * t1315 * t1453 - t2320 * t508 - 4.0_f64 * t2322 * t671 - 2.0_f64 * t2328 * t508 - 4.0_f64 * t2331 * t651 - 2.0_f64 * t2372 * t651 + t3821 * t569 + t4151 * t511;
    (t4151, t4153)
}
