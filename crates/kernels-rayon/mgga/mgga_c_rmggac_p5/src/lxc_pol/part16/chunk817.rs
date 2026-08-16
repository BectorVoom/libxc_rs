//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 817/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk817(t40167: f64, t490: f64, t5011: f64, t511: f64, t270: f64, t38843: f64, t7349: f64, t7351: f64, t2019: f64, t2339: f64, t7926: f64, t118: f64, t2001: f64, t2318: f64, t498: f64) -> (f64, f64, f64, f64, f64) {
    let t40168 = t490 * t40167;
    let t40193 = t5011 * t511;
    let t40198 = t7349 * t7351 * t38843 * t270;
    let t40201 = t2019 * t7926 * t2339;
    let t40231 = t2001 * t118 * t2318 * t498;
    (t40168, t40193, t40198, t40201, t40231)
}
