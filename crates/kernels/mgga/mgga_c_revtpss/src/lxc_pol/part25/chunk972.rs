//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 972/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk972<F: Float>(t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11368: F, t11370: F, t11373: F, t11376: F) -> F {
    let t11500 = -F::new(0.60385000000000000001e0) * t11138 + F::new(0.12077e1) * t11153 + F::new(0.19419375e1) * t11356 - F::new(0.412621875e-1) * t11359 - F::new(0.40256666666666666668e0) * t11134 + F::new(0.30192500000000000001e0) * t11140 + F::new(0.20128333333333333333e0) * t11136 - F::new(0.33547222222222222222e0) * t11147 - F::new(0.301925e0) * t11171 - F::new(0.27595e0) * t11366 + F::new(0.16557e0) * t11368 + F::new(0.258925e1) * t11370 - F::new(0.3883875e1) * t11373 + F::new(0.247573125e0) * t11376;
    t11500
}
