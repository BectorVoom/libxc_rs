//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1152/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1152(t122282: f64, t7063: f64, t7286: f64, t32677: f64, t686: f64, t72: f64, t32705: f64, t32710: f64, t136: f64, t2457: f64, t8708: f64, t119971: f64, t32275: f64, t555: f64) -> (f64, f64, f64, f64, f64) {
    let t122284 = t7063 * t122282 * t7286;
    let t122287 = t32677 * t72 * t686;
    let t122288 = t32705 * t122287;
    let t122290 = t32710 * t122287;
    let t122295 = t8708 * t136 * t2457;
    let t122297 = 0.6019057092162847523e-2_f64 * t119971 * t555 * t32275 * t122295;
    (t122284, t122288, t122290, t122295, t122297)
}
