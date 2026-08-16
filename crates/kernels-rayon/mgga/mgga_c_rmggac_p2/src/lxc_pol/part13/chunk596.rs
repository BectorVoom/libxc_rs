//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 596/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk596(t7206: f64, t7788: f64, t305: f64, t7779: f64, t7769: f64, t797: f64, t7578: f64, t645: f64, t833: f64, t793: f64, t7444: f64, t7707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7789 = t7788 * t7206;
    let t7793 = t305 * t7779;
    let t7795 = t797 * t7769;
    let t7796 = 0.23948483403727617128e0_f64 * t7795;
    let t7797 = t305 * t7578;
    let t7810 = t645 * t833;
    let t7811 = t793 * t7810;
    let t7813 = t797 * t7444;
    let t7815 = t793 * t7707;
    (t7789, t7793, t7795, t7796, t7797, t7810, t7811, t7813, t7815)
}
