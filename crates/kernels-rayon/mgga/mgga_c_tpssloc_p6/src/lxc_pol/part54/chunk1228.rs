//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1228/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1228(t1912: f64, t2054: f64, t25188: f64, t25348: f64, t26700: f64, t30640: f64, t31321: f64, t32791: f64, t32794: f64, t32811: f64, t32817: f64, t33372: f64, t33399: f64, t33416: f64, t33420: f64, t33423: f64, t33430: f64, t33433: f64, t33463: f64, t4147: f64, t4268: f64, t7087: f64, t7538: f64, t855: f64, t8553: f64, t8563: f64) -> f64 {
    let t33465 = -0.82246703342411321825e-2_f64 * t33372 - t32791 - t32794 - t4268 * t8563 - t855 * t33399 - t31321 - t25348 * t2054 + t32811 - t30640 + t32817 + t33416 - t4147 * t8563 - 0.16449340668482264365e-1_f64 * t33420 - 0.82246703342411321825e-2_f64 * t33423 - t7087 * t7538 - t25188 * t2054 + 0.82246703342411321825e-2_f64 * t33430 + 2.0_f64 * t855 * t33433 + 2.0_f64 * t4147 * t8553 + 2.0_f64 * t4268 * t8553 - t26700 * t1912 + t33463;
    t33465
}
