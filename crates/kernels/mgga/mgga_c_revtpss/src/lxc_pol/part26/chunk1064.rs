//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1064/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1064<F: Float>(t94471: F, t94473: F, t94476: F, t94483: F, t94456: F, t94460: F, t94462: F, t94464: F, t94466: F, t94468: F, t94479: F, t94481: F, t94485: F, t94487: F, t94522: F, t94525: F) -> (F, F, F) {
    let t96321 = 455.0 / 648.0 * t94471;
    let t96322 = 0.51384669507166276316e-2 * t94473;
    let t96323 = 0.3252886739816735289e-3 * t94476;
    let t96326 = 0.18295201011342718161e-3 * t94483;
    let t96329 = -0.24009450146119052704e-1 * t94456 - 0.68026775414003982662e-1 * t94460 - 0.85748036236139473944e-3 * t94462 + 0.51448821741683684367e-1 * t94464 - 0.85748036236139473944e-3 * t94466 - 0.15246000842785598468e-3 * t94468 - t96321 + t96322 - t96323 + 0.12196800674228478774e-3 * t94479 + 3.0 / 8.0 * t94481 + t96326 + 7.0 / 24.0 * t94485 - t94487 / 24.0;
    let t96341 = 0.15117061203111996147e0 * t94522;
    let t96342 = 0.80328230880474379779e-6 * t94525;
    (t96329, t96341, t96342)
}
