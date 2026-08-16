//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1350/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1350(t10432: f64, t13969: f64, t3039: f64, t1021: f64, t1025: f64, t1041: f64, t1044: f64, t1046: f64, t10863: f64, t248: f64, t3043: f64, t3064: f64, t3130: f64, t3131: f64, t369: f64, t378: f64, t41671: f64, t42422: f64, t42729: f64, t42731: f64, t42735: f64, t42743: f64, t42746: f64, t42752: f64, t42756: f64, t43083: f64, t43094: f64, t68: f64) -> f64 {
    let t43097 = t3039 * t13969 * t10432;
    let t43099 = -5.0_f64 / 216.0_f64 * t10863 * t3064 + t42729 / 576.0_f64 + t42731 / 72.0_f64 + t42735 / 2304.0_f64 + t1041 * t248 * t1044 * t41671 / 4608.0_f64 - t42743 * t3043 / 512.0_f64 + t42746 * t1046 / 1152.0_f64 + t42752 / 3888.0_f64 + t42756 * t1025 / 768.0_f64 + t43083 * t68 * t369 * t378 / 3072.0_f64 + t3130 * t248 * t1021 * t42422 * t3131 / 512.0_f64 + t43094 / 192.0_f64 - t43097 / 384.0_f64;
    t43099
}
