//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 868/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk868<F: Float>(t8010: F, t8028: F, t8031: F, t8036: F, t8039: F, t8040: F, t8044: F, t8047: F, t153: F, t156: F, t168: F, t242: F, t245: F, t3373: F, t4091: F, t4095: F, t4099: F, t4106: F, t4113: F, t5892: F, t5894: F, t5898: F, t5904: F, t5907: F, t7035: F, t7038: F, t7043: F, t7046: F, t7387: F, t7856: F) -> (F, F) {
    let t8050 = t8010 + t8028 + t8031 + t8036 + t8039 + t8040 + t8044 + t8047;
    let t8068 = -F::cast_from(0.011938374665504766_f64) * t168 * t245 * t8050 + F::cast_from(0.42708890021612717_f64) * t153 * t156 * t7387 - t3373 + t4091 - t4095 - t4099 + t4106 + t4113 - F::cast_from(1.7083556008645087_f64) * t7038 + F::cast_from(0.05969187332752383_f64) * t7043 + F::cast_from(3.9861630686838536_f64) * t5904 + F::cast_from(0.2512884616065132_f64) * t7046 - F::cast_from(0.15917832887339686_f64) * t5907 - F::cast_from(0.2512884616065132_f64) * t7035 - F::cast_from(0.5025769232130264_f64) * t5894 - F::cast_from(0.0837628205355044_f64) * t7856 * t242 + F::cast_from(0.5025769232130264_f64) * t5898 - F::cast_from(0.2512884616065132_f64) * t5892;
    (t8050, t8068)
}
