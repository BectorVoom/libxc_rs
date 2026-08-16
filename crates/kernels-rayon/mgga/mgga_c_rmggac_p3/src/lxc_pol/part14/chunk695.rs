//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 695/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk695(t236: f64, t9182: f64, t7248: f64, t3351: f64, t107: f64, t500: f64, t490: f64) -> (f64, f64, f64) {
    let t9183 = t236 * t9182;
    let t9184 = t7248 * t9183;
    let t9185 = t3351 * t9184;
    let t9187 = t500 * t107;
    let t9188 = t490 * t9187;
    (t9184, t9185, t9188)
}
