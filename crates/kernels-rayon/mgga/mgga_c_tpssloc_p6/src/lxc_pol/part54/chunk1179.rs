//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1179/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1179(t1352: f64, t31636: f64, t31584: f64, t553: f64, t1332: f64, t1336: f64, t31192: f64, t31197: f64, t31200: f64, t31205: f64, t31209: f64, t31617: f64, t31621: f64, t31625: f64, t31629: f64, t31633: f64, t544: f64, t8634: f64) -> (f64, f64, f64) {
    let t31637 = t31636 * t1352;
    let t31639 = t553 * t31584;
    let t31641 = -t31192 - t31197 - t31200 - t31205 + t31209 - t31617 - 0.16449340668482264365e-1_f64 * t31621 - t31625 - 0.82246703342411321825e-2_f64 * t31629 + 0.82246703342411321825e-2_f64 * t31633 + t1332 * t8634 - t1336 * t31637 + t544 * t31639;
    (t31637, t31639, t31641)
}
