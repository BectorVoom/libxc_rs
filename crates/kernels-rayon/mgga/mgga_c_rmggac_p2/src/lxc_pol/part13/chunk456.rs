//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 456/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk456(t1653: f64, t321: f64, t1685: f64, t68: f64, t131: f64, t117: f64, t504: f64) -> (f64, f64, f64, f64) {
    let t4952 = t1653 * t321;
    let t4961 = t68 * t1685;
    let t4962 = t4961 * t131;
    let t4965 = t504 * t117;
    (t4952, t4961, t4962, t4965)
}
