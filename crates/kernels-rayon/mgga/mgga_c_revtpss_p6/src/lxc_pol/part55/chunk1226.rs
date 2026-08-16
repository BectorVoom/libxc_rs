//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1226/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1226(t102854: f64, t127193: f64, t127199: f64, t127207: f64, t127893: f64, t127929: f64, t127940: f64, t127942: f64, t127948: f64, t1940: f64, t26425: f64, t27799: f64, t27800: f64, t27817: f64, t28460: f64, t32080: f64, t32491: f64, t33: f64, t7432: f64, t8677: f64) -> f64 {
    let t128121 = -t127929 - t1940 * t7432 * t127207 / 2.0_f64 - t1940 * t102854 * t8677 / 2.0_f64 + 3.0_f64 * t26425 * t27799 * t127942 + t127940 * t27800 - t1940 * t28460 * t32080 / 2.0_f64 - t1940 * t32491 * t27817 / 2.0_f64 + t1940 * t127893 * t33 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t127193 + t127948 - 3.0_f64 / 2.0_f64 * t26425 * t127199;
    t128121
}
