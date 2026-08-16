//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1095/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1095(t15107: f64, t2741: f64, t4978: f64, t837: f64, t11641: f64, t11647: f64, t11659: f64, t11688: f64, t11692: f64, t11697: f64, t11703: f64, t15058: f64, t15062: f64, t15066: f64, t15071: f64, t15079: f64, t15084: f64, t15089: f64, t15093: f64, t15097: f64, t15102: f64, t2722: f64, t2731: f64, t2740: f64, t8514: f64, t8559: f64, t8568: f64, t9042: f64, t946: f64, t967: f64) -> f64 {
    let t15108 = t2741 * t15107;
    let t15111 = t4978 * t837;
    let t15112 = t2741 * t15111;
    let t15115 = -t2731 * t15058 / 3072.0_f64 - t967 * t15062 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t967 * t15066 - t11641 / 648.0_f64 + t11647 + t9042 + t2740 * t15071 / 4608.0_f64 + t946 * t15079 / 3072.0_f64 - t11659 + t2722 * t15084 / 768.0_f64 + t8559 * t15089 / 512.0_f64 - t8568 * t15093 / 512.0_f64 - t2740 * t15097 / 1152.0_f64 + t8514 * t15102 / 1152.0_f64 - t11688 / 6912.0_f64 - t11692 / 10368.0_f64 - t11697 + t11703 - t2740 * t15108 / 2304.0_f64 + t8514 * t15112 / 2304.0_f64;
    t15115
}
