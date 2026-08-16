//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 959/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk959(t10119: f64, t1625: f64, t2566: f64, t305: f64, t1303: f64, t5814: f64, t9986: f64, t1435: f64, t2568: f64, t10001: f64, t10101: f64, t10105: f64, t10108: f64, t10116: f64, t1451: f64, t1615: f64, t1629: f64, t311: f64, t5085: f64, t5718: f64, t5752: f64, t5757: f64, t5760: f64, t5762: f64, t9980: f64, t9983: f64, t9989: f64, t9995: f64, t9998: f64) -> f64 {
    let t10120 = t10119 * t1625;
    let t10124 = t2566 * t305;
    let t10125 = t1303 * t10124;
    let t10128 = t9986 * t5814;
    let t10132 = t2568 * t1435;
    let t10138 = -t10101 * t311 / 6.0_f64 - t10105 * t311 / 6.0_f64 - t10108 * t311 / 6.0_f64 + 0.1110086767380779_f64 * t9980 - 0.07400578449205193_f64 * t9983 + 0.14975624337724558_f64 * t9995 + 0.29951248675449116_f64 * t9998 - 0.07400578449205193_f64 * t10001 - t1615 * t10116 / 6.0_f64 + t10120 / 6.0_f64 + t2568 * t1629 / 6.0_f64 - t10125 * t1451 / 6.0_f64 - t10128 / 6.0_f64 + t9989 * t5718 / 6.0_f64 + t10132 / 18.0_f64 + t5752 / 6.0_f64 + t5757 - t5760 / 18.0_f64 + t5762 / 18.0_f64 - 0.02466859483068398_f64 * t5085;
    t10138
}
