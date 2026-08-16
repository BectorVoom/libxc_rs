//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1077/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1077(t2286: f64, t9090: f64, t10082: f64, t236: f64, t3351: f64, t35312: f64, t498: f64, t2186: f64, t9935: f64, t1970: f64, t1971: f64, t29439: f64) -> (f64, f64, f64, f64) {
    let t47646 = t9090 * t2286;
    let t47653 = t3351 * t35312 * t236 * t10082 * t498;
    let t47663 = t2186 * t9935;
    let t47667 = t1970 * t1971 * t236 * t29439;
    (t47646, t47653, t47663, t47667)
}
