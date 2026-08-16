//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1012/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1012(t1971: f64, t3351: f64, t6400: f64, t880: f64, t7720: f64, t9938: f64, t16043: f64, t9975: f64, t1704: f64, t236: f64, t35155: f64, t498: f64) -> (f64, f64, f64, f64) {
    let t47156 = t3351 * t1971 * t880 * t6400;
    let t47158 = t7720 * t9938;
    let t47162 = t16043 * t9975;
    let t47167 = t3351 * t35155 * t236 * t1704 * t498;
    (t47156, t47158, t47162, t47167)
}
