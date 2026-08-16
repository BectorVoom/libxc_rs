//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1040/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1040(t102801: f64, t1992: f64, t550: f64, t6976: f64, t102587: f64, t102562: f64, t1985: f64, t1998: f64, t214: f64, t29286: f64, t115391: f64, t122460: f64, t122462: f64, t127356: f64, t127357: f64, t128626: f64, t1814: f64, t33291: f64, t544: f64, t553: f64) -> f64 {
    let t128823 = t1992 * t6976 * t102801 * t550;
    let t128829 = t1992 * t6976 * t102587 * t550;
    let t128833 = t1992 * t6976 * t102562 * t550;
    let t128839 = t1985 * t214 * t1998 * t29286;
    let t128841 = t127356 + 0.82246703342411321824e-2_f64 * t122460 + t127357 + 0.38381794893125283518e-1_f64 * t122462 - 0.82246703342411321825e-2_f64 * t128823 + 2.0_f64 * t1814 * t33291 - t115391 - 0.82246703342411321825e-2_f64 * t128829 - 0.16449340668482264365e-1_f64 * t128833 + t544 * t553 * t128626 + 0.82246703342411321825e-2_f64 * t128839;
    t128841
}
