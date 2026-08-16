//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 826/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk826(t118: f64, t7417: f64, t16503: f64, t2281: f64, t7461: f64, t1357: f64, t34976: f64, t7448: f64, t34975: f64, t35039: f64, t7455: f64, t34761: f64, t9165: f64) -> (f64, f64, f64, f64) {
    let t38508 = t7417 * t118;
    let t38511 = t16503 * t38508 * t2281 * t7461;
    let t38515 = t16503 * t34976 * t1357 * t7448;
    let t38519 = t34975 * t35039 * t2281 * t7455;
    let t38521 = t34761 * t9165;
    (t38511, t38515, t38519, t38521)
}
