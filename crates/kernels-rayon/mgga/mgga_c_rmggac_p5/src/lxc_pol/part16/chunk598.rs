//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 598/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk598(t236: f64, t8455: f64, t1971: f64, t7453: f64, t529: f64, t7754: f64) -> (f64, f64, f64) {
    let t8456 = t236 * t8455;
    let t8457 = t1971 * t8456;
    let t8458 = t7453 * t8457;
    let t8465 = t7754 * t529;
    (t8457, t8458, t8465)
}
