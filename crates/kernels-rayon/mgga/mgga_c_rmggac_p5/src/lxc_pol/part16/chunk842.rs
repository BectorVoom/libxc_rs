//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 842/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk842(t22: f64, t235: f64, t34812: f64, t1982: f64, t2314: f64, t35512: f64, t2289: f64, t7921: f64, t6355: f64, t7707: f64, t1550: f64, t41548: f64) -> (f64, f64, f64, f64, f64) {
    let t41738 = t235 * t34812 * t22;
    let t41767 = t2314 * t35512 * t1982;
    let t41774 = t7921 * t2289;
    let t41789 = t6355 * t7707;
    let t41791 = t1550 * t41548;
    (t41738, t41767, t41774, t41789, t41791)
}
