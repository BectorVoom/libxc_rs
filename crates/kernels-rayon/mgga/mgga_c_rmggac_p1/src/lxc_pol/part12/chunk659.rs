//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 659/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk659(t7603: f64, t8743: f64, t27: f64, t3819: f64, t8747: f64, t3851: f64, t8751: f64, t7599: f64, t8754: f64, t8645: f64, t3839: f64, t8641: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8759 = t7603 * t8743;
    let t8761 = t3819 * t27;
    let t8762 = t8761 * t8747;
    let t8764 = t3851 * t27;
    let t8765 = t8764 * t8751;
    let t8767 = t7599 * t8754;
    let t8769 = t3851 * t8645;
    let t8771 = t3839 * t8641;
    (t8759, t8761, t8762, t8764, t8765, t8767, t8769, t8771)
}
