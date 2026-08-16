//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 566/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk566(t2141: f64, t7501: f64, t649: f64, t848: f64, t27: f64, t2139: f64, t2144: f64, t504: f64) -> (f64, f64, f64, f64) {
    let t7502 = t7501 * t2141;
    let t7504 = t649 * t848;
    let t7505 = t27 * t7504;
    let t7506 = t2139 * t7505;
    let t7508 = t504 * t2144;
    (t7502, t7505, t7506, t7508)
}
