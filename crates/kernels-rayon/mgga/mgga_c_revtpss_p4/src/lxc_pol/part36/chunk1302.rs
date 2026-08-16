//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1302/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1302(t1949: f64, t23167: f64, t105974: f64, t105976: f64, t106121: f64, t106123: f64, t106129: f64, t106275: f64, t1579: f64, t25317: f64, t25416: f64, t2723: f64, t29654: f64, t7070: f64, t7775: f64, t92875: f64, t93138: f64, t98858: f64, t98868: f64, t98875: f64, t98920: f64, t99166: f64) -> (f64, f64) {
    let t113141 = t1949 * t23167;
    let t113160 = t92875 - 0.51405703062096148814e-2_f64 * t98858 - 0.68549505033305214441e-2_f64 * t98868 - 0.26020884564615598386e1_f64 * t7070 * t25416 * t113141 * t2723 + 0.13010442282307799194e0_f64 * t105974 - 0.23132566377943266966e0_f64 * t105976 + 0.68549505033305214441e-2_f64 * t98875 + 0.19514881078765566038e-2_f64 * t98920 - 0.78062653693846795158e1_f64 * t7070 * t25317 * t29654 * t1579 + 0.13010442282307799193e1_f64 * t106275 * t7775 + 0.77108554593144223218e-1_f64 * t106121 - 0.43368140941025997312e-1_f64 * t106123 + 0.15421710918628844643e0_f64 * t106129 - 0.21951497276451705329e-1_f64 * t99166 + t93138;
    (t113141, t113160)
}
