//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1274/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1274<F: Float>(t1085: F, t2343: F, t4: F, t11469: F, t11471: F, t11474: F, t15404: F, t85: F, t8438: F, t1077: F, t5967: F, t8450: F, t1081: F, t6055: F, t15478: F, t8414: F, t8417: F, t8423: F, t8427: F, t8432: F, t8437: F, t8445: F, t8449: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18965 = t2343 * t4 * t1085;
    let t18966 = 0.010843580882781523 * t18965;
    let t18967 = 24.0 * t11469;
    let t18968 = 80.0 * t11471;
    let t18969 = 2.0 * t11474;
    let t18971 = 0.019751789702565206 * t15404 * t85;
    let t18972 = 7.017868076946245 * t8438;
    let t18973 = t5967 * t1077;
    let t18974 = 1.169644679491041 * t18973;
    let t18975 = 24.0 * t8450;
    let t18976 = t6055 * t1081;
    let t18977 = 0.0002441540671567088 * t18976;
    let t18978 = t8414 + t8417 + t15478 + t18966 - t18967 + t18968 + t18969 + t8423 - t8427 + t18971 + t8432 + t8437 - t18972 + t8445 - t8449 + t18974 - t18975 + t18977;
    (t18966, t18967, t18968, t18969, t18971, t18972, t18974, t18975, t18977, t18978)
}
