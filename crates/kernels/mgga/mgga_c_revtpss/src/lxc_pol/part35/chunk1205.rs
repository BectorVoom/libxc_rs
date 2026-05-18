//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1205/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1205<F: Float>(t102409: F, t102411: F, t102434: F, t102439: F, t102462: F, t102582: F, t102656: F, t114636: F, t114660: F, t115166: F, t25930: F, t26079: F, t26304: F, t27868: F, t28911: F, t30105: F, t30252: F, t4003: F, t7295: F, t86413: F, t86641: F, t96473: F, t96491: F, t97933: F) -> F {
    let t115238 = -F::new(0.26020884564615598386e1) * t7295 * t26079 * t115166 * t4003 - F::new(0.28912093960683998208e-1) * t102409 + F::new(0.51405703062096148812e-1) * t102411 + F::new(0.39029762157531132076e-1) * t102434 - F::new(0.34697458558045176417e-2) * t102439 - F::new(0.52041769129231196772e1) * t97933 * t30252 + F::new(0.21951497276451705329e-1) * t102462 + F::new(0.19514881078765566038e-2) * t102582 - F::new(0.26020884564615598386e1) * t25930 * t26304 * t114636 - F::new(0.26020884564615598386e1) * t27868 * t28911 * t86413 + F::new(0.52041769129231196772e1) * t25930 * t28911 * t114660 - t96473 - F::new(0.52041769129231196772e1) * t25930 * t102656 * t30105 + t96491 + F::new(0.13010442282307799193e1) * t27868 * t26304 * t86641;
    t115238
}
