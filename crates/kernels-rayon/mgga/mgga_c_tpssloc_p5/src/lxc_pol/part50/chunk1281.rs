//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1281/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1281(t26103: f64, t7461: f64, t25980: f64, t6517: f64, t26179: f64, t8327: f64, t31058: f64, t7458: f64, t652: f64, t6534: f64, t7670: f64, t19456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120714 = t26103 * t7461;
    let t120716 = t6517 * t25980;
    let t120719 = 2.0_f64 * t26179 * t8327;
    let t120721 = 2.0_f64 * t7458 * t31058;
    let t120723 = t652 * t7670 * t6534;
    let t120728 = 2.0_f64 * t19456 * t8327;
    (t120714, t120716, t120719, t120721, t120723, t120728)
}
