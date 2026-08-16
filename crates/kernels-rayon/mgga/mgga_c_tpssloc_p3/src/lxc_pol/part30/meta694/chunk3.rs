//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2217/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2217(t10110: f64, t17056: f64, t25168: f64, t25169: f64, t25233: f64, t25330: f64, t259: f64, t2713: f64, t28317: f64, t4142: f64, t4147: f64, t4268: f64, t5636: f64, t6662: f64, t7510: f64, t82120: f64, t82123: f64, t855: f64, t92458: f64, t98291: f64, t98305: f64) -> f64 {
    let t98309 = -t92458 + 0.16449340668482264365e-1_f64 * t82120 - t82123 - 6.0_f64 * t25168 * t25169 * t17056 + 0.9869604401089358619e-1_f64 * t98291 + 2.0_f64 * t2713 * t28317 - 6.0_f64 * t855 * t10110 * t6662 * t5636 + 2.0_f64 * t4142 * t7510 * t259 + 4.0_f64 * t4147 * t25233 - 0.16449340668482264365e-1_f64 * t98305 - 2.0_f64 * t4268 * t25330;
    t98309
}
