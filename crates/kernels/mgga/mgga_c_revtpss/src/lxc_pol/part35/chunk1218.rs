//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1218/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1218<F: Float>(t103240: F, t103364: F, t103370: F, t103394: F, t103400: F, t110476: F, t110478: F, t110489: F, t110503: F, t110505: F, t110517: F, t113295: F, t25391: F, t27353: F, t28425: F, t30381: F, t76106: F, t7766: F, t95862: F) -> F {
    let t115614 = -F::new(0.68549505033305214441e-2) * t103240 - F::new(0.21684070470512998656e-1) * t110476 + F::new(0.38554277296572111609e-1) * t110478 + F::new(0.52041769129231196772e1) * t25391 * t28425 * t113295 - F::new(0.26020884564615598386e1) * t27353 * t28425 * t76106 + F::new(0.16463622957338778996e-1) * t110489 + F::new(0.51405703062096148814e-2) * t103364 - F::new(0.19514881078765566037e-2) * t103370 - F::new(0.86736281882051994623e-1) * t110503 + F::new(0.15421710918628844643e0) * t110505 - t95862 - F::new(0.68549505033305214441e-2) * t103394 + F::new(0.43368140941025997312e-1) * t110517 - F::new(0.13010442282307799193e1) * t7766 * t30381 - F::new(0.21951497276451705329e-1) * t103400;
    t115614
}
