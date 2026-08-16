//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 744/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk744(t14366: f64, t27: f64, t684: f64, t2145: f64, t3118: f64, t352: f64, t325: f64, t4616: f64, t235: f64, t3807: f64, t511: f64, t2189: f64, t7228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34805 = t27 * t14366;
    let t34806 = t684 * t34805;
    let t34810 = t2145 * t27 * t3118 * t352;
    let t34812 = t325 * t4616;
    let t34813 = t235 * t34812;
    let t34828 = t3807 * t511;
    let t34846 = t2189 * t7228;
    (t34805, t34806, t34810, t34812, t34813, t34828, t34846)
}
