//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1591/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1591(t14838: f64, t4745: f64, t11350: f64, t11420: f64, t18257: f64, t18261: f64, t18264: f64, t18268: f64, t18631: f64, t18634: f64, t18637: f64, t18640: f64, t18644: f64, t18647: f64, t18651: f64, t18668: f64, t3332: f64, t3357: f64, t436: f64) -> (f64, f64) {
    let t18672 = 4.0_f64 * t14838 * t4745;
    let t18673 = 6.0_f64 * t3357 * t18631 - 4.0_f64 * t3332 * t18634 - 0.19298375398431042081e3_f64 * t11420 * t18637 - 2.0_f64 * t3332 * t18640 + 0.32163958997385070134e2_f64 * t3357 * t18644 + 0.64327917994770140268e2_f64 * t3357 * t18647 + 0.2069040516770936012e4_f64 * t11350 * t18651 - 0.310907e-1_f64 * t18668 * t436 + t18257 - t18261 - t18264 - t18268 + t18672;
    (t18672, t18673)
}
