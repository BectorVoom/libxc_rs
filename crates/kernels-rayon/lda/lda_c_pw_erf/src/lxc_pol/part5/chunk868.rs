//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 868/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk868(t8010: f64, t8028: f64, t8031: f64, t8036: f64, t8039: f64, t8040: f64, t8044: f64, t8047: f64, t153: f64, t156: f64, t168: f64, t242: f64, t245: f64, t3373: f64, t4091: f64, t4095: f64, t4099: f64, t4106: f64, t4113: f64, t5892: f64, t5894: f64, t5898: f64, t5904: f64, t5907: f64, t7035: f64, t7038: f64, t7043: f64, t7046: f64, t7387: f64, t7856: f64) -> (f64, f64) {
    let t8050 = t8010 + t8028 + t8031 + t8036 + t8039 + t8040 + t8044 + t8047;
    let t8068 = -0.011938374665504766_f64 * t168 * t245 * t8050 + 0.42708890021612717_f64 * t153 * t156 * t7387 - t3373 + t4091 - t4095 - t4099 + t4106 + t4113 - 1.7083556008645087_f64 * t7038 + 0.05969187332752383_f64 * t7043 + 3.9861630686838536_f64 * t5904 + 0.2512884616065132_f64 * t7046 - 0.15917832887339686_f64 * t5907 - 0.2512884616065132_f64 * t7035 - 0.5025769232130264_f64 * t5894 - 0.0837628205355044_f64 * t7856 * t242 + 0.5025769232130264_f64 * t5898 - 0.2512884616065132_f64 * t5892;
    (t8050, t8068)
}
