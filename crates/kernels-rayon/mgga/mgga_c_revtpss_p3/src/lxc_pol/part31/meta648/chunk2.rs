//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2138/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2138(t18498: f64, t27159: f64, t1468: f64, t4537: f64, t106546: f64, t106555: f64, t106562: f64, t106566: f64, t106569: f64, t1940: f64, t2403: f64, t25206: f64, t25440: f64, t27158: f64, t27364: f64, t27368: f64, t27382: f64, t27395: f64, t27402: f64, t29592: f64, t29606: f64, t29713: f64, t29719: f64, t50080: f64, t7087: f64, t7091: f64, t7749: f64, t7783: f64, t93404: f64) -> f64 {
    let t106572 = t27159 * t18498;
    let t106583 = t1468 * t4537;
    let t106588 = 6.0_f64 * t27158 * t106546 + 3.0_f64 * t50080 * t29592 + 3.0_f64 * t2403 * t27364 * t7749 + 2.0_f64 * t27382 * t106555 - t1940 * t25440 * t29719 / 2.0_f64 + 3.0_f64 * t25206 * t106562 - 3.0_f64 * t27382 * t106566 - 3.0_f64 * t27158 * t106569 + 6.0_f64 * t27158 * t106572 + 3.0_f64 * t2403 * t7783 * t27395 + t1940 * t93404 * t29713 + 3.0_f64 / 2.0_f64 * t2403 * t7087 * t29606 - t1940 * t7091 * t106583 - t1940 * t27368 * t27402;
    t106588
}
