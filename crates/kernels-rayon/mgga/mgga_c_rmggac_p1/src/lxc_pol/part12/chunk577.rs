//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 577/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk577(t7577: f64, t876: f64, t739: f64, t262: f64, t830: f64, t661: f64, t3826: f64, t7199: f64, t36: f64, t833: f64, t2115: f64, t848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7578 = t7577 * t876;
    let t7579 = t739 * t7578;
    let t7580 = 0.2993560425465952141e-1_f64 * t7579;
    let t7581 = t262 * t830;
    let t7582 = t661 * t7581;
    let t7583 = 0.14784062966376104158e-3_f64 * t7582;
    let t7584 = t3826 * t7199;
    let t7586 = t36 * t833;
    let t7587 = t262 * t7586;
    let t7588 = t2115 * t7587;
    let t7590 = t36 * t848;
    (t7578, t7580, t7581, t7583, t7584, t7586, t7587, t7588, t7590)
}
