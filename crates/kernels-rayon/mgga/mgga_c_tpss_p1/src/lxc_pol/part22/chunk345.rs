//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 345/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk345(t1015: f64, t1128: f64, t242: f64, t1097: f64, t1098: f64, t1103: f64, t1111: f64, t1116: f64, t1122: f64, t1125: f64) -> (f64, f64) {
    let t1129 = t1128 * t1015;
    let t1130 = t242 * t1129;
    let t1133 = t1097 - t1098 * t1103 / 288.0_f64 + t1111 * t1116 / 3072.0_f64 + t1122 - t1125 * t1130 / 4608.0_f64;
    (t1130, t1133)
}
