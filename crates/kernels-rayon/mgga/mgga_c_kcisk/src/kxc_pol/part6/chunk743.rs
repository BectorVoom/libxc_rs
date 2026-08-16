//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 743/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk743(t15200: f64, t15202: f64, t854: f64, t60: f64, t12671: f64, t3140: f64, t979: f64, t3077: f64, t3141: f64, t15176: f64, t15179: f64, t15181: f64, t15183: f64, t15187: f64, t15191: f64, t15195: f64, t15198: f64, t2932: f64) -> (f64, f64, f64, f64) {
    let t15203 = t15200 * t15202;
    let t15206 = t854 * t854;
    let t15207 = 1.0_f64 / t15206;
    let t15208 = t60 * t15207;
    let t15211 = t12671 * t3140;
    let t15212 = t979 * t15211;
    let t15214 = t3077 * t3141;
    let t15216 = -0.29847499999999999999e-1_f64 * t15176 - 0.29847499999999999999e-1_f64 * t15179 + 0.79593333333333333331e-1_f64 * t15181 + 0.39796666666666666665e-1_f64 * t15183 - 0.59694999999999999999e-1_f64 * t15187 + 0.99491666666666666664e-2_f64 * t15191 + 0.92858888888888888885e-1_f64 * t15195 - 0.92858888888888888885e-1_f64 * t15198 - 0.223494e0_f64 * t2932 * t15203 - 0.43134342e-1_f64 * t15208 * t15203 + 0.59694999999999999999e-1_f64 * t15212 - 0.79593333333333333331e-1_f64 * t15214;
    (t15203, t15212, t15214, t15216)
}
