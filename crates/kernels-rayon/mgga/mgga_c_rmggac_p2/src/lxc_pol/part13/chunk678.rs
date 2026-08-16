//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 678/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk678(t7934: f64, t9081: f64, t7933: f64, t1392: f64, t202: f64, t461: f64, t674: f64) -> (f64, f64, f64, f64, f64) {
    let t9082 = t7934 * t9081;
    let t9083 = t7933 * t9082;
    let t9085 = t1392 * t202;
    let t9086 = t9085 * t461;
    let t9087 = t9086 * t674;
    (t9082, t9083, t9085, t9086, t9087)
}
