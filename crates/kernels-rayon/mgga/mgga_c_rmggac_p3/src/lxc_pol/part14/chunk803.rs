//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 803/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk803(t34557: f64, t34558: f64, t7362: f64, t7369: f64, t7373: f64, t7378: f64, t7382: f64, t9758: f64, t9759: f64, t9760: f64, t9761: f64, t8494: f64) -> (f64, f64) {
    let t38230 = t34557 - t34558 - t7362 - t9758 + t9759 - t9760 + t9761 + t7369 - t7373 + t7378 - t7382;
    let t38234 = 0.85129199786595678796e-5_f64 * t8494;
    (t38230, t38234)
}
