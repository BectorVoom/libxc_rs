//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 341/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk341(t132: f64, t31: f64, t2034: f64, t793: f64, t333: f64, t645: f64, t797: f64, t338: f64, t36: f64) -> (f64, f64, f64, f64, f64) {
    let t2051 = t132 * t31;
    let t2055 = t793 * t2034;
    let t2057 = t645 * t333;
    let t2058 = t797 * t2057;
    let t2060 = t338 * t36;
    (t2051, t2055, t2057, t2058, t2060)
}
