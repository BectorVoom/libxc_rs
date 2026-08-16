//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1317/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1317(t20974: f64, t23146: f64, t105353: f64, t105366: f64, t105370: f64, t105372: f64, t105376: f64, t87304: f64, t87306: f64, t87345: f64, t98733: f64, t98736: f64, t98738: f64, t98746: f64, t98750: f64, t98774: f64, t98782: f64, t98787: f64, t98791: f64, t98796: f64, t98798: f64) -> f64 {
    let t105381 = t23146 * t20974;
    let t105383 = -0.20186378047070195427e-3_f64 * t105353 + 7.0_f64 / 192.0_f64 * t98733 - 35.0_f64 / 72.0_f64 * t87304 - 0.2034786907144675699e0_f64 * t87306 + 7.0_f64 / 768.0_f64 * t98736 + 7.0_f64 / 384.0_f64 * t98738 + 0.42391393898847410397e-2_f64 * t98746 - 0.12111826828242117256e-2_f64 * t98750 - 0.42391393898847410397e-2_f64 * t98774 - 0.20186378047070195427e-3_f64 * t98782 + 0.10093189023535097714e-3_f64 * t98787 + 0.10093189023535097714e-3_f64 * t98791 - t105366 / 4.0_f64 - 0.67826230238155856634e-1_f64 * t105370 - t105372 / 48.0_f64 - 0.12111826828242117256e-2_f64 * t105376 - 119.0_f64 / 576.0_f64 * t87345 - 7.0_f64 / 384.0_f64 * t98796 + 7.0_f64 / 768.0_f64 * t98798 - 5.0_f64 / 128.0_f64 * t105381;
    t105383
}
