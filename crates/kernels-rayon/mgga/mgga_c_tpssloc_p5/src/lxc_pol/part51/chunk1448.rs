//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1448/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1448(t22892: f64, t22893: f64, t33276: f64, t22751: f64, t33277: f64, t552: f64, t7918: f64, t1307: f64, t6637: f64, t6888: f64, t114104: f64, t114119: f64, t120505: f64, t120506: f64, t120507: f64, t120513: f64, t120515: f64, t120522: f64, t120525: f64, t120526: f64, t122518: f64, t122522: f64, t122526: f64, t122530: f64) -> f64 {
    let t122533 = t22892 * t22893 * t33276;
    let t122535 = t22751 * t33277;
    let t122537 = t552 * t7918;
    let t122540 = t6888 * t6637 * t122537 * t1307;
    let t122542 = 0.16449340668482264365e-1_f64 * t122518 + t120505 - t120506 + t114104 + t120507 + t120513 - t120515 - t120522 + 0.16449340668482264365e-1_f64 * t122522 - 0.16449340668482264365e-1_f64 * t122526 - 0.16449340668482264365e-1_f64 * t122530 + 0.82246703342411321825e-2_f64 * t122533 + 0.38381794893125283518e-1_f64 * t122535 - 0.16449340668482264365e-1_f64 * t122540 - t120525 + t114119 + t120526;
    t122542
}
