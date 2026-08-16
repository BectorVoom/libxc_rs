//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1215/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1215<F: Float>(t2061: F, t23167: F, t103017: F, t103030: F, t103063: F, t103424: F, t106275: F, t110289: F, t110291: F, t110316: F, t110318: F, t110323: F, t113286: F, t113387: F, t231: F, t25391: F, t26550: F, t27353: F, t29682: F, t7070: F, t7076: F, t76161: F, t8007: F, t93349: F, t95732: F) -> (F, F) {
    let t115499 = t2061 * t23167;
    let t115521 = -F::cast_from(0.43368140941025997312e-1_f64) * t110289 + F::cast_from(0.77108554593144223218e-1_f64) * t110291 - F::cast_from(0.72280234901709995519e-3_f64) * t103017 + F::cast_from(0.26020884564615598386e1_f64) * t106275 * t8007 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t115499 * t231 + F::cast_from(0.21684070470512998656e-1_f64) * t110316 - F::cast_from(0.38554277296572111609e-1_f64) * t110318 - F::cast_from(0.34697458558045176417e-2_f64) * t103030 + F::cast_from(0.78062653693846795158e1_f64) * t93349 * t26550 * t113387 - F::cast_from(0.26020884564615598386e1_f64) * t25391 * t26550 * t113286 + F::cast_from(0.13010442282307799193e1_f64) * t27353 * t26550 * t76161 - F::cast_from(0.52041769129231196772e1_f64) * t25391 * t103424 * t29682 + F::cast_from(0.77108554593144223218e-1_f64) * t110323 - t95732 + F::cast_from(0.51405703062096148812e-1_f64) * t103063;
    (t115499, t115521)
}
