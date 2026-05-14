//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1076/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1076<F: Float>(t103017: F, t103030: F, t103063: F, t103424: F, t106275: F, t110289: F, t110291: F, t110316: F, t110318: F, t110323: F, t113286: F, t113387: F, t115499: F, t231: F, t25391: F, t26550: F, t27353: F, t29682: F, t7070: F, t7076: F, t76161: F, t8007: F, t93349: F, t95732: F) -> (F,) {
    let t115521 = -0.43368140941025997312e-1 * t110289 + 0.77108554593144223218e-1 * t110291 - 0.72280234901709995519e-3 * t103017 + 0.26020884564615598386e1 * t106275 * t8007 + 0.4336814094102599731e0 * t7070 * t7076 * t115499 * t231 + 0.21684070470512998656e-1 * t110316 - 0.38554277296572111609e-1 * t110318 - 0.34697458558045176417e-2 * t103030 + 0.78062653693846795158e1 * t93349 * t26550 * t113387 - 0.26020884564615598386e1 * t25391 * t26550 * t113286 + 0.13010442282307799193e1 * t27353 * t26550 * t76161 - 0.52041769129231196772e1 * t25391 * t103424 * t29682 + 0.77108554593144223218e-1 * t110323 - t95732 + 0.51405703062096148812e-1 * t103063;
    (t115521,)
}
