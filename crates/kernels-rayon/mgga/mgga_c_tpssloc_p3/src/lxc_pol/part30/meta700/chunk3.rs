//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2256/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2256(t16949: f64, t221: f64, t25154: f64, t25119: f64, t841: f64, t81921: f64, t81928: f64, t81934: f64, t81943: f64, t81955: f64, t87444: f64, t87445: f64, t87464: f64, t87478: f64, t87488: f64, t98847: f64, t98849: f64, t98851: f64, t98853: f64, t98858: f64, t98862: f64) -> f64 {
    let t98868 = t25154 * t221 * t16949;
    let t98871 = t25119 * t841 * t16949;
    let t98873 = t98847 / 384.0_f64 - 5.0_f64 / 384.0_f64 * t98849 + t98851 / 192.0_f64 - t98853 / 768.0_f64 - t81921 + 119.0_f64 / 6912.0_f64 * t81928 - 0.20186378047070195427e-3_f64 * t98858 + 0.12111826828242117256e-2_f64 * t98862 - 0.67826230238155856634e-1_f64 * t81934 - 35.0_f64 / 216.0_f64 * t81943 + t87444 + 0.20186378047070195427e-3_f64 * t87445 - t81955 - t87464 + t98868 / 16.0_f64 + 0.84782787797694820792e-2_f64 * t98871 - t87478 + t87488;
    t98873
}
