//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1225/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1225(t1592: f64, t30285: f64, t3308: f64, t1054: f64, t6132: f64, t8745: f64, t6139: f64, t8741: f64, t40176: f64, t40178: f64, t40181: f64, t40185: f64, t43602: f64, t43606: f64, t43609: f64, t43612: f64, t43616: f64) -> f64 {
    let t43619 = t1592 * t3308 * t30285;
    let t43622 = t6132 * t1054 * t8745;
    let t43625 = t6139 * t1054 * t8741;
    let t43627 = t40176 + 0.23115257973478049502e0_f64 * t43602 + t40178 + t40181 - 0.31147743054556651236e-1_f64 * t40185 + 0.21831846657716620896e-2_f64 * t43606 - 0.46574606203128791245e-1_f64 * t43609 - 0.26198215989259945075e-1_f64 * t43612 + 0.27944763721877274747e0_f64 * t43616 + 0.13002332610081402845e0_f64 * t43619 - 0.17336443480108537126e0_f64 * t43622 - 0.5200933044032561138e0_f64 * t43625;
    t43627
}
