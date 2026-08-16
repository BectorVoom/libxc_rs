//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2031/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2031(t100858: f64, t103553: f64, t14749: f64, t14767: f64, t15071: f64, t1544: f64, t1583: f64, t1940: f64, t198: f64, t207: f64, t2071: f64, t2394: f64, t2403: f64, t26425: f64, t26581: f64, t26590: f64, t28291: f64, t2832: f64, t28460: f64, t4343: f64, t4433: f64, t4541: f64, t61155: f64, t61182: f64, t63186: f64, t7428: f64, t7432: f64, t8020: f64, t892: f64, t95527: f64, t95964: f64, t98759: f64, t98786: f64) -> f64 {
    let t103658 = 12.0_f64 * t26425 * t100858 - t1940 * t7432 * t15071 + 6.0_f64 * t2403 * t26590 * t61155 - 6.0_f64 * t4541 * t7432 * t98759 + 12.0_f64 * t4541 * t7428 * t4433 + 6.0_f64 * t4541 * t8020 * t2394 - t1940 * t28460 * t2832 + t198 * t207 * t103553 * t892 + 6.0_f64 * t2403 * t7428 * t4343 - 6.0_f64 * t2403 * t7432 * t61182 - 12.0_f64 * t28291 * t63186 + 12.0_f64 * t4541 * t2071 * t14749 + 6.0_f64 * t4541 * t2071 * t14767 - 6.0_f64 * t1940 * t95964 * t98786 - t1940 * t95527 * t1583 + 3.0_f64 * t2403 * t26581 * t1544;
    t103658
}
