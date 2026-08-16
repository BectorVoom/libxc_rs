//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2257/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2257(t29313: f64, t3801: f64, t12587: f64, t8220: f64, t104509: f64, t104560: f64, t104601: f64, t105057: f64, t105107: f64, t105155: f64, t105206: f64, t105258: f64, t105310: f64, t105358: f64, t105402: f64, t105457: f64, t105504: f64, t105553: f64, t105613: f64, t105657: f64, t1298: f64, t1300: f64, t18123: f64, t1832: f64, t198: f64, t27037: f64, t27041: f64, t29317: f64, t29322: f64, t336: f64, t3794: f64, t3798: f64, t5023: f64, t5501: f64, t73262: f64, t7673: f64, t97487: f64, t97491: f64, t97498: f64) -> f64 {
    let t105665 = t29313 * t3801;
    let t105669 = t8220 * t12587;
    let t105696 = t198 * t336 * (t104509 + t104560 + t104601 + t105057 + t105107 + t105155 + t105206 + t105258 + t105310 + t105358 + t105402 + t105457 + t105504 + t105553 + t105613 + t105657) * t1300 - 2.0_f64 * t5023 * t105665 * t1298 + 2.0_f64 * t5023 * t105669 * t3798 - t5023 * t29317 * t3794 - t5023 * t97487 * t1832 + 4.0_f64 * t5023 * t97491 * t29322 - 2.0_f64 * t5023 * t27037 * t5501 - 6.0_f64 * t5023 * t97498 * t1832 * t3798 + 4.0_f64 * t5023 * t27041 * t73262 + 2.0_f64 * t5023 * t27041 * t1832 * t3794 - t5023 * t7673 * t18123;
    t105696
}
