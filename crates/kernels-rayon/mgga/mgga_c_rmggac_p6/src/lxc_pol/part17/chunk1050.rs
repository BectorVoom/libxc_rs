//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1050/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1050(t1528: f64, t236: f64, t3351: f64, t618: f64, t9210: f64, t7720: f64, t9932: f64, t39277: f64, t8836: f64, t8843: f64, t2320: f64, t39281: f64) -> (f64, f64, f64, f64, f64) {
    let t47263 = t3351 * t9210 * t236 * t618 * t1528;
    let t47265 = t7720 * t9932;
    let t47267 = t39277 * t8836;
    let t47269 = t39277 * t8843;
    let t47271 = t39281 * t2320;
    (t47263, t47265, t47267, t47269, t47271)
}
