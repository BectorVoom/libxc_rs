//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 377/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk377(t2231: f64, t82: f64, t302: f64, t702: f64, t2227: f64, t515: f64, t290: f64) -> (f64, f64, f64, f64) {
    let t2232 = t82 * t2231;
    let t2244 = t302 * t702;
    let t2262 = t515 * t2227;
    let t2265 = t290 * t702;
    (t2232, t2244, t2262, t2265)
}
