//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1310/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1310(t1579: f64, t231: f64, t6016: f64, t5977: f64, t106216: f64, t106218: f64, t106236: f64, t106238: f64, t106267: f64, t113141: f64, t25391: f64, t25392: f64, t29698: f64, t7070: f64, t7076: f64, t7779: f64, t93206: f64, t93210: f64, t93224: f64, t93231: f64, t99258: f64, t99261: f64) -> (f64, f64) {
    let t113269 = t1579 * t6016 * t231;
    let t113285 = t1579 * t5977;
    let t113286 = t113285 * t231;
    let t113291 = -0.26020884564615598386e1_f64 * t25391 * t25392 * t113269 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t113141 * t231 + 0.51405703062096148814e-2_f64 * t99258 + 0.72280234901709995519e-3_f64 * t99261 - 0.13010442282307799193e1_f64 * t29698 * t7779 - 0.77108554593144223218e-1_f64 * t106216 + 0.43368140941025997312e-1_f64 * t106218 + t93206 - t93210 + t93224 - 0.21684070470512998656e-1_f64 * t106236 + 0.38554277296572111609e-1_f64 * t106238 - t93231 - 0.26020884564615598386e1_f64 * t25391 * t25392 * t113286 - 0.58544643236296698113e-1_f64 * t106267;
    (t113285, t113291)
}
