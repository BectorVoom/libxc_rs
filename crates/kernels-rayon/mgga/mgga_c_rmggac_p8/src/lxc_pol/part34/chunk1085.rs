//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1085/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1085(t78560: f64, t70517: f64, t70524: f64, t72117: f64, t72119: f64, t76515: f64, t78536: f64, t78540: f64, t78544: f64, t78545: f64, t78546: f64, t78547: f64, t78548: f64, t78551: f64, t78553: f64, t78556: f64, t78557: f64) -> f64 {
    let t78561 = 0.15243824895787514157e-3_f64 * t78560;
    let t78562 = -t78536 + t78540 - t78544 + t78545 + t78546 - t72117 - t78547 + t72119 + t78548 + 0.6505345598561924296e-5_f64 * t70517 + 0.6505345598561924296e-5_f64 * t70524 - t78551 + t78553 - t78556 - t76515 - t78557 + t78561;
    t78562
}
