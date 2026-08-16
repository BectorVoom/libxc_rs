//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 918/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk918(t8577: f64, t9153: f64, t39208: f64, t8457: f64, t1907: f64, t1971: f64, t209: f64, t236: f64, t476: f64, t7453: f64, t2283: f64, t38351: f64) -> (f64, f64, f64, f64) {
    let t45274 = t8577 * t9153;
    let t45277 = t39208 * t8457;
    let t45283 = t7453 * t1971 * t236 * t1907 * t476 * t209;
    let t45285 = t38351 * t2283;
    (t45274, t45277, t45283, t45285)
}
