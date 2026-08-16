//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 944/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk944(t118: f64, t2001: f64, t2318: f64, t498: f64, t7717: f64, t1462: f64, t1971: f64, t333: f64, t511: f64, t8517: f64, t352: f64, t515: f64) -> (f64, f64, f64) {
    let t40231 = t2001 * t118 * t2318 * t498;
    let t40232 = t7717 * t40231;
    let t40237 = t8517 * t1971 * t511 * t1462 * t333;
    let t40242 = t8517 * t1971 * t515 * t1462 * t352;
    (t40232, t40237, t40242)
}
