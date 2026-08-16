//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 951/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk951(t114693: f64, t112998: f64, t113005: f64, t113009: f64, t114670: f64, t114673: f64, t114677: f64, t114680: f64, t114685: f64, t114689: f64, t114691: f64, t234: f64, t7084: f64) -> (f64, f64) {
    let t114694 = 0.63969658155208805863e-1_f64 * t114693;
    let t114695 = -t112998 - 0.38381794893125283518e-1_f64 * t114670 + t114673 + 0.16449340668482264365e-1_f64 * t114677 + 0.82246703342411321824e-2_f64 * t114680 - 0.16449340668482264365e-1_f64 * t114685 - t113005 - t113009 - t114689 - 0.82246703342411321824e-2_f64 * t114691 + t114694;
    let t114696 = t234 * t7084;
    (t114695, t114696)
}
