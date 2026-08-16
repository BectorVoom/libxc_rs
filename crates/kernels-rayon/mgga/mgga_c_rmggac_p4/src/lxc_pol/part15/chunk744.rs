//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 744/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk744(t3350: f64, t34846: f64, t201: f64, t4443: f64, t1976: f64, t674: f64, t2185: f64, t7472: f64, t16155: f64, t7229: f64) -> (f64, f64, f64, f64, f64) {
    let t34847 = t34846 * t3350;
    let t34855 = t201 * t4443;
    let t34857 = t1976 * t34855 * t674;
    let t34881 = t7472 * t2185;
    let t34884 = t7229 * t16155;
    (t34847, t34855, t34857, t34881, t34884)
}
