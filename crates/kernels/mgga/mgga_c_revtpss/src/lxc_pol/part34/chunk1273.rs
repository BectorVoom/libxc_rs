//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1273/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1273<F: Float>(t1949: F, t23167: F, t105974: F, t105976: F, t106121: F, t106123: F, t106129: F, t106275: F, t1579: F, t25317: F, t25416: F, t2723: F, t29654: F, t7070: F, t7775: F, t92875: F, t93138: F, t98858: F, t98868: F, t98875: F, t98920: F, t99166: F) -> (F, F) {
    let t113141 = t1949 * t23167;
    let t113160 = t92875 - F::new(0.51405703062096148814e-2) * t98858 - F::new(0.68549505033305214441e-2) * t98868 - F::new(0.26020884564615598386e1) * t7070 * t25416 * t113141 * t2723 + F::new(0.13010442282307799194e0) * t105974 - F::new(0.23132566377943266966e0) * t105976 + F::new(0.68549505033305214441e-2) * t98875 + F::new(0.19514881078765566038e-2) * t98920 - F::new(0.78062653693846795158e1) * t7070 * t25317 * t29654 * t1579 + F::new(0.13010442282307799193e1) * t106275 * t7775 + F::new(0.77108554593144223218e-1) * t106121 - F::new(0.43368140941025997312e-1) * t106123 + F::new(0.15421710918628844643e0) * t106129 - F::new(0.21951497276451705329e-1) * t99166 + t93138;
    (t113141, t113160)
}
