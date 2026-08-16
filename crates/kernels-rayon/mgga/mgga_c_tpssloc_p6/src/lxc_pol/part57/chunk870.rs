//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 870/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk870(t214: f64, t33284: f64, t1985: f64, t1825: f64, t31636: f64, t33266: f64, t553: f64, t1336: f64, t1814: f64, t31192: f64, t31200: f64, t31617: f64, t31625: f64, t32743: f64, t32747: f64, t32751: f64, t33278: f64, t33282: f64, t544: f64, t8634: f64) -> (f64, f64, f64, f64) {
    let t33285 = t214 * t33284;
    let t33286 = t1985 * t33285;
    let t33289 = t31636 * t1825;
    let t33291 = t553 * t33266;
    let t33293 = -t31192 - t32743 - t31200 - t32747 + t32751 - t31617 - 0.16449340668482264365e-1_f64 * t33278 - t31625 - 0.82246703342411321825e-2_f64 * t33282 + 0.82246703342411321825e-2_f64 * t33286 + t1814 * t8634 - t1336 * t33289 + t544 * t33291;
    (t33285, t33289, t33291, t33293)
}
