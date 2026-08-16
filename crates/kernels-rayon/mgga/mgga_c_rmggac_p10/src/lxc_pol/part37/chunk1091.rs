//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1091/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1091(t75892: f64, t75895: f64, t69201: f64, t69207: f64, t69245: f64, t71269: f64, t71270: f64, t71271: f64, t71272: f64, t75304: f64, t78122: f64, t78123: f64, t78124: f64, t78125: f64, t78126: f64, t78127: f64, t78129: f64, t78130: f64) -> (f64, f64, f64) {
    let t80347 = 0.16566831523319392754e-1_f64 * t75892;
    let t80349 = 0.20439190441718261718e-5_f64 * t75895;
    let t80351 = t69201 - t69207 - t78122 - t78123 + t78124 + t69245 - t78125 + t78126 + t71269 + t71270 - t71271 + t71272 - t78127 + 0.10286123809333192469e-2_f64 * t75304 - t78129 + t78130;
    (t80347, t80349, t80351)
}
