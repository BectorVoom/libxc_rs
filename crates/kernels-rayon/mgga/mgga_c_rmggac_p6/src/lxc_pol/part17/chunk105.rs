//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 105/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk105(t312: f64, t90: f64, t101: f64, t266: f64, t309: f64, t87: f64, t91: f64, t98: f64) -> (f64, f64, f64, f64) {
    let t313 = t90 * t312;
    let t316 = -t312;
    let t317 = t101 * t316;
    let t320 = 2.0_f64 / 3.0_f64 * t266;
    let t321 = -10.0_f64 / 3.0_f64 * t309 * t91 + 10.0_f64 / 3.0_f64 * t87 * t313 + 10.0_f64 / 3.0_f64 * t98 * t317 + t320;
    (t316, t317, t320, t321)
}
