//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 750/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk750(t35238: f64, t2019: f64, t2164: f64, t7352: f64, t7764: f64, t7556: f64, t7553: f64, t7555: f64, t288: f64, t49: f64, t2038: f64, t7756: f64, t7933: f64) -> (f64, f64, f64, f64, f64) {
    let t35239 = 0.45731474687362542471e-3_f64 * t35238;
    let t35242 = t2019 * t7764 * t2164 * t7352;
    let t35244 = t2164 * t7556;
    let t35246 = t7553 * t7555 * t35244;
    let t35253 = t49 * t288;
    let t35256 = t7933 * t2038 * t35253 * t7756;
    (t35239, t35242, t35246, t35253, t35256)
}
