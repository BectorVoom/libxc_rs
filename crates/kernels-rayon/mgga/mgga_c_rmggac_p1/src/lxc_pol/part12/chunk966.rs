//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 966/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk966(t3351: f64, t3352: f64, t511: f64, t5218: f64, t1971: f64, t5184: f64, t880: f64, t2144: f64, t31125: f64, t2010: f64, t8342: f64, t935: f64) -> (f64, f64, f64, f64) {
    let t40533 = t3351 * t3352 * t511 * t5218;
    let t40537 = t3351 * t1971 * t880 * t5184;
    let t40541 = t3351 * t1971 * t2144 * t31125;
    let t40544 = t2010 * t8342 * t935;
    (t40533, t40537, t40541, t40544)
}
