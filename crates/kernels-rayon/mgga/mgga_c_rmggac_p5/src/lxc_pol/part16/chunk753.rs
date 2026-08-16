//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 753/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk753(t2164: f64, t7556: f64, t7553: f64, t7555: f64, t288: f64, t49: f64, t2038: f64, t7756: f64, t7933: f64, t108: f64, t4179: f64, t490: f64) -> (f64, f64, f64, f64) {
    let t35244 = t2164 * t7556;
    let t35246 = t7553 * t7555 * t35244;
    let t35253 = t49 * t288;
    let t35256 = t7933 * t2038 * t35253 * t7756;
    let t35311 = t4179 * t108;
    let t35312 = t490 * t35311;
    (t35246, t35253, t35256, t35312)
}
