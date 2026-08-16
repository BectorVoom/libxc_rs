//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 768/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk768(t25518: f64, t27: f64, t25640: f64, t25636: f64, t25525: f64, t344: f64, t3899: f64, t35875: f64, t793: f64, t35924: f64, t797: f64, t262: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36103 = t25518 * t27;
    let t36107 = t25640 * t27;
    let t36110 = t25636 * t27;
    let t36119 = t25525 * t27;
    let t36156 = t344 * t3899;
    let t36166 = t793 * t35875;
    let t36168 = t797 * t35924;
    let t36172 = t262 * t3899;
    (t36103, t36107, t36110, t36119, t36156, t36166, t36168, t36172)
}
