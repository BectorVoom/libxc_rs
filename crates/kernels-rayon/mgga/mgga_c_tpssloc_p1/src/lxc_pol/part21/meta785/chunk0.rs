//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2719/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2719(t57229: f64, t40227: f64, t40231: f64, t40233: f64, t118: f64, t2375: f64, t6320: f64, t54477: f64, t40224: f64, t40230: f64, t57218: f64, t57219: f64, t57220: f64, t57221: f64, t57222: f64, t57223: f64, t57224: f64, t57225: f64, t57226: f64, t57228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57230 = 8.0_f64 * t57229;
    let t57231 = 24.0_f64 * t40227;
    let t57232 = 12.0_f64 * t40231;
    let t57233 = 32.0_f64 * t40233;
    let t57235 = t6320 * t118 * t2375;
    let t57236 = 0.10843581300301739842e-1_f64 * t57235;
    let t57237 = 8.0_f64 * t54477;
    let t57238 = t57218 - t57219 - t57220 + t57221 - t57222 - t57223 - t57224 - t57225 + t40224 - t57226 + t57228 - t57230 - t57231 - t40230 + t57232 + t57233 + t57236 + t57237;
    (t57230, t57231, t57232, t57233, t57236, t57237, t57238)
}
