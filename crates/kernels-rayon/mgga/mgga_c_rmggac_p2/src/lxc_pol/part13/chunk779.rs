//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 779/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk779(t344: f64, t3899: f64, t265: f64, t5245: f64, t35863: f64, t797: f64, t35875: f64, t793: f64, t35924: f64, t262: f64, t661: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36156 = t344 * t3899;
    let t36158 = t5245 * t265;
    let t36160 = t797 * t35863;
    let t36166 = t793 * t35875;
    let t36168 = t797 * t35924;
    let t36172 = t262 * t3899;
    let t36173 = t661 * t36172;
    let t36175 = t854 * t35863;
    (t36156, t36158, t36160, t36166, t36168, t36172, t36173, t36175)
}
