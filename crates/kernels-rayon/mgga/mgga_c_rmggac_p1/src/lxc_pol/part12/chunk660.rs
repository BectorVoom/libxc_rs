//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 660/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk660(t3826: f64, t8645: f64, t7595: f64, t7597: f64, t7618: f64, t7620: f64, t8759: f64, t8762: f64, t8765: f64, t8767: f64, t8769: f64, t8771: f64) -> f64 {
    let t8773 = t3826 * t8645;
    let t8778 = -0.63504270469206447408e-3_f64 * t8759 + 0.84672360625608596544e-3_f64 * t8762 + 0.68186654135613354324e-2_f64 * t8765 - 0.13637330827122670865e-1_f64 * t8767 + t7595 + 0.2993560425465952141e-1_f64 * t8769 - 0.13276154105060581339e-2_f64 * t8771 + 0.19914231157590872008e-2_f64 * t8773 + 0.2660942600414179681e-1_f64 * t7597 - 0.39914139006212695215e-1_f64 * t7618 + 0.88507694033737208925e-3_f64 * t7620;
    t8778
}
