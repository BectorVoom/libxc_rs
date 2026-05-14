//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 869/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk869<F: Float>(t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11368: F, t11370: F, t11373: F, t11376: F, t11485: F, t973: F) -> (F, F) {
    let t11500 = -0.60385000000000000001e0 * t11138 + 0.12077e1 * t11153 + 0.19419375e1 * t11356 - 0.412621875e-1 * t11359 - 0.40256666666666666668e0 * t11134 + 0.30192500000000000001e0 * t11140 + 0.20128333333333333333e0 * t11136 - 0.33547222222222222222e0 * t11147 - 0.301925e0 * t11171 - 0.27595e0 * t11366 + 0.16557e0 * t11368 + 0.258925e1 * t11370 - 0.3883875e1 * t11373 + 0.247573125e0 * t11376;
    let t11501 = t11485 + t11500;
    let t11502 = t11501 * t973;
    (t11501, t11502)
}
