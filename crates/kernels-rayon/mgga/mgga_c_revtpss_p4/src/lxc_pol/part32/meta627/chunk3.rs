//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2005/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2005(t116: f64, t30552: f64, t1940: f64, t2255: f64, t8020: f64, t105928: f64, t28472: f64, t105902: f64, t105909: f64, t106510: f64, t18280: f64, t2071: f64, t2403: f64, t27169: f64, t27402: f64, t28456: f64, t28460: f64, t29591: f64, t29602: f64, t29606: f64, t29713: f64, t30420: f64, t4541: f64, t7010: f64, t7428: f64, t7432: f64, t7749: f64, t95976: f64) -> (f64, f64, f64, f64) {
    let t110110 = t30552 * t116;
    let t110150 = 2.0_f64 * t1940 * t8020 * t2255;
    let t110154 = 2.0_f64 * t28472 * t105928;
    let t110158 = -t1940 * t7432 * t106510 / 2.0_f64 + 3.0_f64 * t2403 * t2071 * t105909 - t1940 * t28460 * t27402 + 3.0_f64 * t4541 * t2071 * t105902 + 3.0_f64 * t2403 * t8020 * t27169 + 3.0_f64 * t2403 * t28456 * t7749 + t1940 * t2071 * t18280 / 2.0_f64 + 3.0_f64 * t2403 * t7428 * t29602 + 3.0_f64 * t4541 * t7428 * t29591 + 3.0_f64 / 2.0_f64 * t2403 * t30420 * t7010 + t110150 + t1940 * t95976 * t29713 - t110154 + 3.0_f64 / 2.0_f64 * t2403 * t7428 * t29606;
    (t110110, t110150, t110154, t110158)
}
