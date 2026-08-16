//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1218/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1218(t107460: f64, t107464: f64, t107484: f64, t107908: f64, t107928: f64, t107951: f64, t107987: f64, t1375: f64, t1378: f64, t20608: f64, t2091: f64, t29311: f64, t40591: f64, t5215: f64, t84705: f64, t91531: f64, t91548: f64, t97732: f64, t97750: f64) -> f64 {
    let t107993 = 0.19739208802178717238e0_f64 * t107460 + 0.29608813203268075857e0_f64 * t107464 + 0.9869604401089358619e-1_f64 * t97732 + 24.0_f64 * t1375 * t40591 * t2091 * t20608 - 0.15626873635058151147e0_f64 * t91531 + 12.0_f64 * t5215 * t29311 - 0.11514538467937585055e0_f64 * t97750 + 0.9869604401089358619e-1_f64 * t91548 - t1375 * t1378 * (t107908 + t107928 + t107951 + t107987) - t84705 - 0.39478417604357434476e0_f64 * t107484;
    t107993
}
