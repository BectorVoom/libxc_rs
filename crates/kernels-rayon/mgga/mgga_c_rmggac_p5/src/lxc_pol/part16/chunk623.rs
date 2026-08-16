//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 623/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk623(t8751: f64, t8764: f64, t7599: f64, t8754: f64, t3851: f64, t8645: f64, t3839: f64, t8641: f64, t3826: f64, t8625: f64, t3814: f64, t8631: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8765 = t8764 * t8751;
    let t8767 = t7599 * t8754;
    let t8769 = t3851 * t8645;
    let t8771 = t3839 * t8641;
    let t8773 = t3826 * t8645;
    let t8784 = t3851 * t8625;
    let t8786 = t3814 * t8631;
    (t8765, t8767, t8769, t8771, t8773, t8784, t8786)
}
