//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3174/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3174(t57242: f64, t57251: f64, t57257: f64, t57271: f64, t57274: f64, t57331: f64, t57548: f64, t59330: f64, t70511: f64, t70521: f64, t70523: f64, t70542: f64, t77513: f64) -> f64 {
    let t83361 = -0.28582678745379824648e-3_f64 * t70511 - 0.42874018118069736972e-3_f64 * t70521 + 0.85748036236139473944e-3_f64 * t70523 - t57242 + t57251 + t57257 - 0.42874018118069736972e-3_f64 * t70542 - t57271 + t57274 + 0.95275595817932748825e-4_f64 * t57331 - t57548 * t59330 * t77513 / 16.0_f64;
    t83361
}
