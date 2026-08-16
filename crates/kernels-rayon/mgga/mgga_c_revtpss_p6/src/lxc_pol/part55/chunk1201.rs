//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1201/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1201(t2411: f64, t34079: f64, t102888: f64, t106589: f64, t121751: f64, t127566: f64, t1940: f64, t2403: f64, t26585: f64, t27160: f64, t27166: f64, t27395: f64, t27402: f64, t28460: f64, t28472: f64, t32487: f64, t32491: f64, t32499: f64, t32505: f64, t32508: f64, t34080: f64, t34100: f64, t605: f64, t7092: f64, t7749: f64, t8657: f64) -> (f64, f64) {
    let t127582 = t34079 * t2411;
    let t127592 = 3.0_f64 / 2.0_f64 * t2403 * t8657 * t27395 - 3.0_f64 / 2.0_f64 * t102888 * t32499 + 3.0_f64 * t127566 * t27160 + 3.0_f64 / 2.0_f64 * t2403 * t32487 * t7749 - t1940 * t28460 * t32508 / 2.0_f64 + t28472 * t106589 * t32505 - 3.0_f64 / 2.0_f64 * t121751 * t27166 - t1940 * t32491 * t27402 / 2.0_f64 - t1940 * t127582 * t7092 / 2.0_f64 + t1940 * t34080 * t605 / 2.0_f64 - t1940 * t26585 * t34100 / 2.0_f64;
    (t127582, t127592)
}
