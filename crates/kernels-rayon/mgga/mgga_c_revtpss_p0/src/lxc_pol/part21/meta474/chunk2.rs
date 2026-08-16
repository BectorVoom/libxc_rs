//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2039/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2039(t14997: f64, t15022: f64, t15044: f64, t15069: f64, t2430: f64, t4542: f64, t10596: f64, t10604: f64, t10611: f64, t14436: f64, t14442: f64, t14443: f64, t14444: f64, t14468: f64, t14615: f64, t14618: f64, t14620: f64, t14621: f64, t14624: f64, t14626: f64, t14628: f64, t14629: f64, t1940: f64, t198: f64, t207: f64, t2404: f64, t2408: f64, t4433: f64, t4541: f64, t765: f64, t892: f64, t9542: f64) -> (f64, f64, f64) {
    let t15071 = t14997 + t15022 + t15044 + t15069;
    let t15078 = t4542 * t2430;
    let t15081 = t15071 * t198 * t207 * t892 + 2.0_f64 * t14436 * t1940 * t2408 + 3.0_f64 * t14468 * t198 * t765 + 12.0_f64 * t2404 * t4433 * t4541 + 6.0_f64 * t15078 * t4541 - t10596 - t10604 - t10611 + t14442 - t14443 - t14444 + t14615 - t14618 + t14620 + t14621 + t14624 + t14626 + t14628 + t14629 + t9542;
    (t15071, t15078, t15081)
}
