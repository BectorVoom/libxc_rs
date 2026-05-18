//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1207/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1207<F: Float>(t94471: F, t94473: F, t94476: F, t94483: F, t94456: F, t94460: F, t94462: F, t94464: F, t94466: F, t94468: F, t94479: F, t94481: F, t94485: F, t94487: F) -> F {
    let t96321 = F::new(455.0) / F::new(648.0) * t94471;
    let t96322 = F::new(0.51384669507166276316e-2) * t94473;
    let t96323 = F::new(0.3252886739816735289e-3) * t94476;
    let t96326 = F::new(0.18295201011342718161e-3) * t94483;
    let t96329 = -F::new(0.24009450146119052704e-1) * t94456 - F::new(0.68026775414003982662e-1) * t94460 - F::new(0.85748036236139473944e-3) * t94462 + F::new(0.51448821741683684367e-1) * t94464 - F::new(0.85748036236139473944e-3) * t94466 - F::new(0.15246000842785598468e-3) * t94468 - t96321 + t96322 - t96323 + F::new(0.12196800674228478774e-3) * t94479 + F::new(3.0) / F::new(8.0) * t94481 + t96326 + F::new(7.0) / F::new(24.0) * t94485 - t94487 / F::new(24.0);
    t96329
}
