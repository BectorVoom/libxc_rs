//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1098/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1098(t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11171: f64, t11356: f64, t11359: f64, t11366: f64, t11368: f64, t11370: f64, t11373: f64, t11376: f64) -> f64 {
    let t11500 = -0.60385000000000000001e0_f64 * t11138 + 0.12077e1_f64 * t11153 + 0.19419375e1_f64 * t11356 - 0.412621875e-1_f64 * t11359 - 0.40256666666666666668e0_f64 * t11134 + 0.30192500000000000001e0_f64 * t11140 + 0.20128333333333333333e0_f64 * t11136 - 0.33547222222222222222e0_f64 * t11147 - 0.301925e0_f64 * t11171 - 0.27595e0_f64 * t11366 + 0.16557e0_f64 * t11368 + 0.258925e1_f64 * t11370 - 0.3883875e1_f64 * t11373 + 0.247573125e0_f64 * t11376;
    t11500
}
