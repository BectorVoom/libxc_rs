//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 674/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk674(t242: f64, t4100: f64, t1198: f64, t632: f64, t1143: f64, t458: f64, t2853: f64, t41: f64, t1203: f64, t1155: f64, t153: f64, t156: f64, t168: f64, t245: f64, t3196: f64, t3373: f64, t3375: f64, t3378: f64, t4079: f64, t4084: f64, t4087: f64, t4091: f64, t4092: f64, t4095: f64, t4096: f64, t4099: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4101 = t4100 * t242;
    let t4103 = t1198 * t632;
    let t4106 = 0.2512884616065132_f64 * t458 * t1143;
    let t4107 = t41 * t2853;
    let t4110 = t1203 * t632;
    let t4113 = 0.5025769232130264_f64 * t1155 * t242;
    let t4114 = 0.42708890021612717_f64 * t153 * t156 * t3196 - t3373 - 1.7083556008645087_f64 * t3375 + 3.9861630686838536_f64 * t3378 - 0.011938374665504766_f64 * t168 * t245 * t4079 - 0.15917832887339686_f64 * t4084 + 0.05969187332752383_f64 * t4087 + t4091 - 0.2512884616065132_f64 * t4092 - t4095 - 0.5025769232130264_f64 * t4096 - t4099 + 0.2512884616065132_f64 * t4101 + 0.5025769232130264_f64 * t4103 + t4106 - 0.0837628205355044_f64 * t4107 * t242 - 0.2512884616065132_f64 * t4110 + t4113;
    (t4101, t4103, t4106, t4107, t4110, t4113, t4114)
}
