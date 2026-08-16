//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2208/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2208(t1455: f64, t8249: f64, t116: f64, t29421: f64, t10416: f64, t13425: f64, t13435: f64, t13540: f64, t13544: f64, t1502: f64, t18163: f64, t2163: f64, t2322: f64, t2331: f64, t27056: f64, t27060: f64, t27079: f64, t29427: f64, t29444: f64, t29456: f64, t4246: f64, t4248: f64, t4254: f64, t4292: f64, t4297: f64, t651: f64, t671: f64, t7586: f64, t7683: f64, t8158: f64, t97604: f64, t97606: f64, t97608: f64) -> (f64, f64, f64) {
    let t104094 = 2.0_f64 * t1455 * t8249;
    let t104115 = t29421 * t116;
    let t104135 = -4.0_f64 * t4292 * t651 * t7683 - 4.0_f64 * t104115 * t671 - 2.0_f64 * t10416 * t8158 - t13425 * t2163 - 4.0_f64 * t13435 * t8158 - 4.0_f64 * t13540 * t7586 - 2.0_f64 * t13544 * t7586 - t1502 * t27056 - 2.0_f64 * t18163 * t8158 - 4.0_f64 * t2322 * t29444 - 4.0_f64 * t2322 * t29456 - 4.0_f64 * t2331 * t29427 - 4.0_f64 * t27060 * t4297 - 2.0_f64 * t27079 * t4248 - 4.0_f64 * t29444 * t4254 - 2.0_f64 * t4246 * t7683 - t97604 - t97606 - t97608;
    (t104094, t104115, t104135)
}
