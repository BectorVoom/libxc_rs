//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 967/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk967(t1390: f64, t3914: f64, t3719: f64, t571: f64, t12048: f64, t12051: f64, t12053: f64, t12055: f64, t12057: f64, t12059: f64, t12085: f64, t12087: f64, t12090: f64, t12092: f64, t12094: f64, t1307: f64, t3918: f64, t5126: f64, t9789: f64, t9793: f64) -> f64 {
    let t12466 = t3914 * t1390;
    let t12470 = t571 * t3719;
    let t12474 = 9.0_f64 * t12466 * t1307 * t3918 + 18.0_f64 * t12470 * t1307 * t5126 - t12048 + t12051 + t12053 + t12055 - t12057 - t12059 + t12085 + t12087 - t12090 - t12092 - t12094 - t9789 + t9793;
    t12474
}
