//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 411/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk411(t131: f64, t4028: f64, t1346: f64, t49: f64, t288: f64, t325: f64, t504: f64) -> (f64, f64, f64, f64) {
    let t4029 = t4028 * t131;
    let t4035 = t1346 * t49;
    let t4036 = t4035 * t288;
    let t4041 = t504 * t325;
    (t4029, t4035, t4036, t4041)
}
