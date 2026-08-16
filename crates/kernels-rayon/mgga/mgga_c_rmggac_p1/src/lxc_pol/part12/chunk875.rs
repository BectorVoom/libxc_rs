//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 875/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk875(t1212: f64, t1970: f64, t209: f64, t236: f64, t3352: f64, t551: f64, t1971: f64, t5578: f64, t495: f64, t7230: f64, t9210: f64, t9211: f64) -> (f64, f64, f64) {
    let t39215 = t1970 * t3352 * t236 * t551 * t1212 * t209;
    let t39219 = t1970 * t1971 * t236 * t5578;
    let t39224 = t7230 * t9210 * t236 * t9211 * t495;
    (t39215, t39219, t39224)
}
