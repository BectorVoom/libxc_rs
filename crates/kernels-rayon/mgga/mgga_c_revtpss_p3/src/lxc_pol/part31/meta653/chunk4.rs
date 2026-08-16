//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2180/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2180(t6416: f64, t775: f64, t106501: f64, t27799: f64, t25759: f64, t77441: f64, t1711: f64, t4537: f64, t106539: f64, t1113: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25440: f64, t27364: f64, t27773: f64, t27777: f64, t29705: f64, t29940: f64, t29946: f64, t29967: f64, t50080: f64, t7091: f64, t7200: f64, t7783: f64, t7862: f64, t7869: f64, t92819: f64, t99555: f64) -> f64 {
    let t107970 = t6416 * t775;
    let t107974 = t27799 * t106501;
    let t107985 = t25759 * t77441;
    let t107988 = t1711 * t4537;
    let t108001 = 3.0_f64 * t50080 * t29940 + 3.0_f64 / 2.0_f64 * t2403 * t29705 * t7200 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t107970 + 6.0_f64 * t25206 * t107974 + 3.0_f64 * t2403 * t7783 * t27773 - t1940 * t99555 * t7869 - t106539 + 3.0_f64 * t2403 * t27364 * t7862 - 3.0_f64 * t25206 * t107985 - t1940 * t7091 * t107988 - 3.0_f64 * t92819 * t29946 + t1940 * t29705 * t1113 / 2.0_f64 - t1940 * t25440 * t29967 + 3.0_f64 * t2403 * t7783 * t27777;
    t108001
}
