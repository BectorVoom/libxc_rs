//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1012/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1012(t13087: f64, t13182: f64, t13190: f64, t13202: f64, t13208: f64, t13234: f64, t13237: f64, t13262: f64, t16836: f64, t16841: f64, t16845: f64, t16848: f64, t16853: f64, t16859: f64, t2618: f64, t4172: f64, t4178: f64, t4184: f64, t4257: f64, t5587: f64, t5614: f64, t5619: f64, t817: f64, t843: f64, t9602: f64, t9672: f64, t9967: f64) -> f64 {
    let t16869 = -35.0_f64 / 108.0_f64 * t13087 - 119.0_f64 / 3456.0_f64 * t9602 - 119.0_f64 / 6912.0_f64 * t13182 - t13190 + t13202 - t13208 + t16836 * t4184 / 768.0_f64 - t13262 * t16841 / 512.0_f64 + t4178 * t16845 / 512.0_f64 - 7.0_f64 / 576.0_f64 * t16848 - 119.0_f64 / 13824.0_f64 * t9672 - 5.0_f64 / 128.0_f64 * t843 * t16853 - t2618 * t5614 / 3072.0_f64 - t817 * t16859 / 3072.0_f64 - t2618 * t5619 / 3072.0_f64 + 5.0_f64 / 384.0_f64 * t4172 * t4257 + 119.0_f64 / 6912.0_f64 * t13234 - t13237 + t9967 * t5587 / 1536.0_f64;
    t16869
}
