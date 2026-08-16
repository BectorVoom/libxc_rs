//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 959/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk959<F: Float>(t10119: F, t1625: F, t2566: F, t305: F, t1303: F, t5814: F, t9986: F, t1435: F, t2568: F, t10001: F, t10101: F, t10105: F, t10108: F, t10116: F, t1451: F, t1615: F, t1629: F, t311: F, t5085: F, t5718: F, t5752: F, t5757: F, t5760: F, t5762: F, t9980: F, t9983: F, t9989: F, t9995: F, t9998: F) -> F {
    let t10120 = t10119 * t1625;
    let t10124 = t2566 * t305;
    let t10125 = t1303 * t10124;
    let t10128 = t9986 * t5814;
    let t10132 = t2568 * t1435;
    let t10138 = -t10101 * t311 / F::cast_from(6.0_f64) - t10105 * t311 / F::cast_from(6.0_f64) - t10108 * t311 / F::cast_from(6.0_f64) + F::cast_from(0.1110086767380779_f64) * t9980 - F::cast_from(0.07400578449205193_f64) * t9983 + F::cast_from(0.14975624337724558_f64) * t9995 + F::cast_from(0.29951248675449116_f64) * t9998 - F::cast_from(0.07400578449205193_f64) * t10001 - t1615 * t10116 / F::cast_from(6.0_f64) + t10120 / F::cast_from(6.0_f64) + t2568 * t1629 / F::cast_from(6.0_f64) - t10125 * t1451 / F::cast_from(6.0_f64) - t10128 / F::cast_from(6.0_f64) + t9989 * t5718 / F::cast_from(6.0_f64) + t10132 / F::cast_from(18.0_f64) + t5752 / F::cast_from(6.0_f64) + t5757 - t5760 / F::cast_from(18.0_f64) + t5762 / F::cast_from(18.0_f64) - F::cast_from(0.02466859483068398_f64) * t5085;
    t10138
}
