//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2409/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2409(t42087: f64, t47787: f64, t59700: f64, t59702: f64, t59704: f64, t60274: f64, t68619: f64, t68626: f64, t68628: f64, t68630: f64, t68633: f64, t68635: f64) -> f64 {
    let t68864 = t42087 - 0.3560484375e1_f64 * t68619 + 0.5477111111111111111e-1_f64 * t60274 - 0.11958666666666666667e1_f64 * t59700 + 0.39862222222222222222e0_f64 * t59702 + 0.33218518518518518518e0_f64 * t59704 + 0.93011851851851851854e0_f64 * t47787 + 0.427258125e1_f64 * t68626 - 0.28483875e1_f64 * t68628 - 0.28483875e1_f64 * t68630 + 0.1151859375e0_f64 * t68633 - 0.230371875e0_f64 * t68635;
    t68864
}
