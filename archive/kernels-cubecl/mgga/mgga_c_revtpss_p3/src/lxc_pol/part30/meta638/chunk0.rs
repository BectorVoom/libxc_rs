//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2208/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2208<F: Float>(t1455: F, t8249: F, t116: F, t29421: F, t10416: F, t13425: F, t13435: F, t13540: F, t13544: F, t1502: F, t18163: F, t2163: F, t2322: F, t2331: F, t27056: F, t27060: F, t27079: F, t29427: F, t29444: F, t29456: F, t4246: F, t4248: F, t4254: F, t4292: F, t4297: F, t651: F, t671: F, t7586: F, t7683: F, t8158: F, t97604: F, t97606: F, t97608: F) -> (F, F, F) {
    let t104094 = F::cast_from(2.0_f64) * t1455 * t8249;
    let t104115 = t29421 * t116;
    let t104135 = -F::cast_from(4.0_f64) * t4292 * t651 * t7683 - F::cast_from(4.0_f64) * t104115 * t671 - F::cast_from(2.0_f64) * t10416 * t8158 - t13425 * t2163 - F::cast_from(4.0_f64) * t13435 * t8158 - F::cast_from(4.0_f64) * t13540 * t7586 - F::cast_from(2.0_f64) * t13544 * t7586 - t1502 * t27056 - F::cast_from(2.0_f64) * t18163 * t8158 - F::cast_from(4.0_f64) * t2322 * t29444 - F::cast_from(4.0_f64) * t2322 * t29456 - F::cast_from(4.0_f64) * t2331 * t29427 - F::cast_from(4.0_f64) * t27060 * t4297 - F::cast_from(2.0_f64) * t27079 * t4248 - F::cast_from(4.0_f64) * t29444 * t4254 - F::cast_from(2.0_f64) * t4246 * t7683 - t97604 - t97606 - t97608;
    (t104094, t104115, t104135)
}
