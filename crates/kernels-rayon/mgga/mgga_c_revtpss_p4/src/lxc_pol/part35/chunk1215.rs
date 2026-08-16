//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1215/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1215(t2061: f64, t23167: f64, t103017: f64, t103030: f64, t103063: f64, t103424: f64, t106275: f64, t110289: f64, t110291: f64, t110316: f64, t110318: f64, t110323: f64, t113286: f64, t113387: f64, t231: f64, t25391: f64, t26550: f64, t27353: f64, t29682: f64, t7070: f64, t7076: f64, t76161: f64, t8007: f64, t93349: f64, t95732: f64) -> (f64, f64) {
    let t115499 = t2061 * t23167;
    let t115521 = -0.43368140941025997312e-1_f64 * t110289 + 0.77108554593144223218e-1_f64 * t110291 - 0.72280234901709995519e-3_f64 * t103017 + 0.26020884564615598386e1_f64 * t106275 * t8007 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t115499 * t231 + 0.21684070470512998656e-1_f64 * t110316 - 0.38554277296572111609e-1_f64 * t110318 - 0.34697458558045176417e-2_f64 * t103030 + 0.78062653693846795158e1_f64 * t93349 * t26550 * t113387 - 0.26020884564615598386e1_f64 * t25391 * t26550 * t113286 + 0.13010442282307799193e1_f64 * t27353 * t26550 * t76161 - 0.52041769129231196772e1_f64 * t25391 * t103424 * t29682 + 0.77108554593144223218e-1_f64 * t110323 - t95732 + 0.51405703062096148812e-1_f64 * t103063;
    (t115499, t115521)
}
