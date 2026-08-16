//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1370/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1370(t5: f64, t122975: f64, t123020: f64, t112: f64, t119810: f64, t119811: f64, t119824: f64, t119826: f64, t119830: f64, t119831: f64, t119835: f64, t122914: f64, t122918: f64, t122921: f64, t122923: f64, t122925: f64, t510: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t123022 = piecewise3(t8, 0.0_f64, t122975 + t123020);
    let t123023 = t123022 * t112;
    let t123025 = -t123023 * t510 - t119810 - 2.0_f64 * t119811 - t119824 - t119826 - t119830 + t119831 + t119835 + 3.0_f64 * t122914 - 2.0_f64 * t122918 - 2.0_f64 * t122921 - 2.0_f64 * t122923 - t122925;
    (t123023, t123025)
}
