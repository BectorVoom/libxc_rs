//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1226/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1226(t105353: f64, t105366: f64, t105370: f64, t105372: f64, t105376: f64, t105381: f64, t87304: f64, t87306: f64, t87345: f64, t98733: f64, t98736: f64, t98738: f64, t98746: f64, t98750: f64, t98774: f64, t98782: f64, t98787: f64, t98791: f64, t98796: f64, t98798: f64) -> f64 {
    let t108290 = -0.40372756094140390853e-3_f64 * t105353 + 7.0_f64 / 96.0_f64 * t98733 - 35.0_f64 / 36.0_f64 * t87304 - 0.4069573814289351398e0_f64 * t87306 + 7.0_f64 / 384.0_f64 * t98736 + 7.0_f64 / 192.0_f64 * t98738 + 0.84782787797694820791e-2_f64 * t98746 - 0.24223653656484234512e-2_f64 * t98750 - 0.84782787797694820791e-2_f64 * t98774 - 0.40372756094140390854e-3_f64 * t98782 + 0.20186378047070195427e-3_f64 * t98787 + 0.20186378047070195427e-3_f64 * t98791 - t105366 / 2.0_f64 - 0.13565246047631171326e0_f64 * t105370 - t105372 / 24.0_f64 - 0.24223653656484234512e-2_f64 * t105376 - 119.0_f64 / 288.0_f64 * t87345 - 7.0_f64 / 192.0_f64 * t98796 + 7.0_f64 / 384.0_f64 * t98798 - 5.0_f64 / 64.0_f64 * t105381;
    t108290
}
