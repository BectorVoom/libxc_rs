//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1379/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1379(t114795: f64, t114811: f64, t114815: f64, t118640: f64, t118810: f64, t118814: f64, t118825: f64, t121444: f64, t121448: f64, t121451: f64, t121454: f64, t121457: f64, t1528: f64, t25170: f64, t2597: f64, t26729: f64, t33399: f64, t866: f64) -> f64 {
    let t121462 = 0.41123351671205660912e-2_f64 * t114795 - 0.82246703342411321825e-2_f64 * t121444 + 0.16449340668482264365e-1_f64 * t121448 - t114811 * t1528 - 6.0_f64 * t121451 * t25170 - t121454 * t866 - t118810 - t114815 - 0.82246703342411321825e-2_f64 * t121457 - t2597 * t33399 - 6.0_f64 * t118640 * t26729 + t118814 + t118825;
    t121462
}
