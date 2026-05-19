//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1281/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1281<F: Float>(t1579: F, t231: F, t6016: F, t5977: F, t106216: F, t106218: F, t106236: F, t106238: F, t106267: F, t113141: F, t25391: F, t25392: F, t29698: F, t7070: F, t7076: F, t7779: F, t93206: F, t93210: F, t93224: F, t93231: F, t99258: F, t99261: F) -> (F, F) {
    let t113269 = t1579 * t6016 * t231;
    let t113285 = t1579 * t5977;
    let t113286 = t113285 * t231;
    let t113291 = -F::cast_from(0.26020884564615598386e1_f64) * t25391 * t25392 * t113269 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t113141 * t231 + F::cast_from(0.51405703062096148814e-2_f64) * t99258 + F::cast_from(0.72280234901709995519e-3_f64) * t99261 - F::cast_from(0.13010442282307799193e1_f64) * t29698 * t7779 - F::cast_from(0.77108554593144223218e-1_f64) * t106216 + F::cast_from(0.43368140941025997312e-1_f64) * t106218 + t93206 - t93210 + t93224 - F::cast_from(0.21684070470512998656e-1_f64) * t106236 + F::cast_from(0.38554277296572111609e-1_f64) * t106238 - t93231 - F::cast_from(0.26020884564615598386e1_f64) * t25391 * t25392 * t113286 - F::cast_from(0.58544643236296698113e-1_f64) * t106267;
    (t113285, t113291)
}
