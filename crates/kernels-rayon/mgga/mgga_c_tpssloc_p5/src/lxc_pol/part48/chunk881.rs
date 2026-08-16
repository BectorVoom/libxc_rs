//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 881/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk881(t2165: f64, t7056: f64, t2040: f64, t2314: f64, t24932: f64, t31294: f64, t31296: f64, t31298: f64, t31302: f64, t32318: f64, t32359: f64, t4034: f64, t574: f64, t652: f64, t7042: f64, t7057: f64, t7061: f64, t7266: f64, t7271: f64, t8835: f64) -> (f64, f64) {
    let t32365 = t2165 * t7056;
    let t32368 = -2.0_f64 * t2040 * t24932 - 2.0_f64 * t2314 * t8835 - 2.0_f64 * t32318 * t652 + t32359 * t574 - 2.0_f64 * t32365 * t652 - 2.0_f64 * t4034 * t8835 - 2.0_f64 * t7042 * t7271 - 2.0_f64 * t7057 * t7266 - 2.0_f64 * t7061 * t7266 + t31294 - t31296 - t31298 - t31302;
    (t32365, t32368)
}
