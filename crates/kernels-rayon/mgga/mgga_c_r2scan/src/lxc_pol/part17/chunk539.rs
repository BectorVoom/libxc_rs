//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 539/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk539(t1288: f64, t2901: f64, t2905: f64, t2917: f64, t2921: f64, t313: f64) -> f64 {
    let t2938 = 3.0_f64 / 10.0_f64 * t313 * (10.0_f64 / 9.0_f64 * t2901 + 5.0_f64 / 3.0_f64 * t2905 + 10.0_f64 / 9.0_f64 * t2917 + 5.0_f64 / 3.0_f64 * t2921) + t1288;
    t2938
}
