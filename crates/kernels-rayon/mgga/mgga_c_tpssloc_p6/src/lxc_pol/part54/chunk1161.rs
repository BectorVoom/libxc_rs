//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1161/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1161(t1912: f64, t2054: f64, t23278: f64, t23281: f64, t24297: f64, t259: f64, t2597: f64, t2713: f64, t30626: f64, t30637: f64, t30640: f64, t30645: f64, t31311: f64, t31317: f64, t31321: f64, t31347: f64, t31350: f64, t31351: f64, t31362: f64, t31368: f64, t31371: f64, t31400: f64, t31427: f64, t6627: f64, t6632: f64, t7087: f64, t7092: f64, t855: f64, t8563: f64) -> f64 {
    let t31429 = t30626 + 2.0_f64 * t855 * t31311 + 0.82246703342411321825e-2_f64 * t31317 - t31321 + 2.0_f64 * t6627 * t7092 + t30637 + 2.0_f64 * t7087 * t6632 - t30640 + t30645 - t24297 * t1912 + t31347 - t31350 + t31351 * t259 + t31362 * t259 - t2597 * t8563 - t2713 * t8563 - 0.16449340668482264365e-1_f64 * t31368 - 0.82246703342411321825e-2_f64 * t31371 - t23278 * t2054 - t855 * t31400 - t23281 * t2054 + t31427;
    t31429
}
