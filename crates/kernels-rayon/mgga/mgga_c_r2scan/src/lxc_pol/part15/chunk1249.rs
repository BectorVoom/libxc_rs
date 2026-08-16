//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1249/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1249(t1039: f64, t1044: f64, t11028: f64, t1108: f64, t11166: f64, t12019: f64, t40666: f64, t40670: f64, t40679: f64, t40683: f64, t40686: f64, t40690: f64, t40694: f64, t40699: f64, t40704: f64, t40708: f64, t40711: f64, t40715: f64, t40717: f64, t8505: f64, t885: f64) -> f64 {
    let t41098 = t1039 * t11028 + t1044 * t11166 + t1108 * t8505 + 2.0_f64 * t12019 * t885 + t40666 - t40670 + t40679 + t40683 + t40686 + t40690 - t40694 + t40699 - t40704 - t40708 + t40711 + t40715 - t40717;
    t41098
}
