//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1185/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1185(t3452: f64, t5172: f64, t10787: f64, t5062: f64, t14097: f64, t5047: f64, t5046: f64, t10506: f64, t251: f64, t14611: f64, t5180: f64, t15061: f64, t15063: f64, t15066: f64, t15069: f64, t15072: f64, t15074: f64, t15076: f64) -> (f64, f64, f64, f64, f64) {
    let t15078 = t5172 * t3452;
    let t15080 = t10787 * t5062;
    let t15082 = t5047 * t14097;
    let t15083 = t5046 * t15082;
    let t15085 = t251 * t10506;
    let t15086 = t15085 * t14611;
    let t15087 = t5180 * t15086;
    let t15089 = -t15061 / 192.0_f64 + t15063 / 18.0_f64 - t15066 / 64.0_f64 - t15069 / 12.0_f64 - 11.0_f64 / 18.0_f64 * t15072 - t15074 / 24.0_f64 - t15076 / 24.0_f64 + t15078 / 256.0_f64 + 2.0_f64 / 9.0_f64 * t15080 + t15083 / 8.0_f64 + t15087 / 864.0_f64;
    (t15078, t15080, t15083, t15087, t15089)
}
