//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 948/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk948<F: Float>(t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11368: F, t11370: F, t11373: F, t11376: F) -> F {
    let t11443 = -F::new(0.103295e1) * t11138 + F::new(0.20659e1) * t11153 + F::new(0.264729375e1) * t11356 - F::new(0.157790625e0) * t11359 - F::new(0.68863333333333333332e0) * t11134 + F::new(0.51647499999999999999e0) * t11140 + F::new(0.34431666666666666666e0) * t11136 - F::new(0.57386111111111111112e0) * t11147 - F::new(0.516475e0) * t11171 - F::new(0.34731666666666666667e0) * t11366 + F::new(0.20839e0) * t11368 + F::new(0.3529725e1) * t11370 - F::new(0.52945875e1) * t11373 + F::new(0.94674375e0) * t11376;
    t11443
}
