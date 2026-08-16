//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1005/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1005(t5055: f64, t7769: f64, t1341: f64, t575: f64, t638: f64, t7310: f64, t7244: f64, t8427: f64, t2144: f64, t3351: f64, t352: f64, t7231: f64, t8502: f64) -> (f64, f64, f64, f64) {
    let t42034 = t5055 * t7769;
    let t42042 = t638 * t7310 * t575 * t1341;
    let t42044 = t7244 * t8427;
    let t42050 = t3351 * t7231 * t2144 * t8502 * t352;
    (t42034, t42042, t42044, t42050)
}
