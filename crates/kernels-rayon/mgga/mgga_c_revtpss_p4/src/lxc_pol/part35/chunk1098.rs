//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1098/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1098(t1923: f64, t2048: f64, t26175: f64, t26207: f64, t28154: f64, t28598: f64, t28600: f64, t28602: f64, t28628: f64, t28638: f64, t28641: f64, t29513: f64, t29538: f64, t29544: f64, t29548: f64, t29551: f64, t29554: f64, t29562: f64, t30543: f64, t7343: f64, t7702: f64, t7706: f64, t7709: f64, t7964: f64) -> f64 {
    let t30551 = -10.0_f64 / 3.0_f64 * t28602 * t7706 - 4.0_f64 / 3.0_f64 * t29538 * t2048 - 10.0_f64 / 3.0_f64 * t7343 * t29544 - 5.0_f64 / 3.0_f64 * t7343 * t29548 - 2.0_f64 / 3.0_f64 * t29551 * t2048 - 2.0_f64 / 3.0_f64 * t29554 * t2048 - 4.0_f64 / 3.0_f64 * t7709 * t7964 + 80.0_f64 / 9.0_f64 * t28598 + 32.0_f64 / 9.0_f64 * t28600 - 16.0_f64 / 9.0_f64 * t28638 + t29513 * t2048 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7702 * t7964 + t1923 * t30543 / 3.0_f64 - 16.0_f64 / 9.0_f64 * t28641 + 10.0_f64 * t26175 * t29562 + 20.0_f64 / 3.0_f64 * t28154 * t28628 + t26207;
    t30551
}
