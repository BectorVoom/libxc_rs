//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1218/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1218(t103240: f64, t103364: f64, t103370: f64, t103394: f64, t103400: f64, t110476: f64, t110478: f64, t110489: f64, t110503: f64, t110505: f64, t110517: f64, t113295: f64, t25391: f64, t27353: f64, t28425: f64, t30381: f64, t76106: f64, t7766: f64, t95862: f64) -> f64 {
    let t115614 = -0.68549505033305214441e-2_f64 * t103240 - 0.21684070470512998656e-1_f64 * t110476 + 0.38554277296572111609e-1_f64 * t110478 + 0.52041769129231196772e1_f64 * t25391 * t28425 * t113295 - 0.26020884564615598386e1_f64 * t27353 * t28425 * t76106 + 0.16463622957338778996e-1_f64 * t110489 + 0.51405703062096148814e-2_f64 * t103364 - 0.19514881078765566037e-2_f64 * t103370 - 0.86736281882051994623e-1_f64 * t110503 + 0.15421710918628844643e0_f64 * t110505 - t95862 - 0.68549505033305214441e-2_f64 * t103394 + 0.43368140941025997312e-1_f64 * t110517 - 0.13010442282307799193e1_f64 * t7766 * t30381 - 0.21951497276451705329e-1_f64 * t103400;
    t115614
}
