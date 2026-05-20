//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1654/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1654<F: Float>(t3475: F, t426: F, t3478: F, t1179: F, t12378: F, t3488: F, t3520: F, t1161: F, t1169: F, t1180: F, t1188: F, t1189: F, t12465: F, t12470: F, t12472: F, t12473: F, t12476: F, t12481: F, t12486: F, t12488: F, t12494: F, t12548: F, t12553: F, t3447: F, t3479: F, t3480: F, t3491: F, t3498: F, t3516: F, t3523: F, t3524: F, t43753: F, t43961: F, t45057: F, t45061: F, t45064: F, t45075: F, t45080: F, t45085: F, t45103: F, t45118: F, t45134: F, t45149: F) -> F {
    let t45155 = t3475 * t3475;
    let t45157 = t426 / t45155;
    let t45158 = t3478 * t3478;
    let t45159 = F::new(1.0) / t45158;
    let t45163 = t12378 * t1179;
    let t45168 = t3488 * t3520;
    let t45173 = F::cast_from(0.5848223622634646207e0_f64) * t1180 * t43961 * t1188 + F::cast_from(0.11579025239058625248e4_f64) * t12470 * t45057 * t3479 - F::cast_from(0.70178683471615754484e1_f64) * t45061 * t3498 - F::cast_from(0.4155806185363551302e3_f64) * t45064 * t12488 + F::cast_from(0.6233709278045326953e3_f64) * t12553 * t43753 * t3523 + F::cast_from(0.14035736694323150897e2_f64) * t12481 * t12494 - F::cast_from(0.14035736694323150897e2_f64) * t12486 * t43753 * t1188 + F::cast_from(0.1929837539843104208e3_f64) * t45075 * t3480 + F::new(4.0) * t3447 * t12465 + F::cast_from(0.82761620670837440481e4_f64) * t45080 * t12473 - F::cast_from(0.24828486201251232145e5_f64) * t45085 * t45057 * t12472 + F::new(1.0) * t1161 * (t45103 + t45118 + t45134 + t45149) * t1169 + F::cast_from(0.19964560303604640732e6_f64) * t45157 * t45057 * t45159 + F::cast_from(0.23392894490538584828e1_f64) * t45163 * t1189 + F::cast_from(0.35089341735807877242e1_f64) * t12476 * t3516 + F::cast_from(0.10389515463408878255e3_f64) * t45168 * t3524 + F::cast_from(0.23392894490538584828e1_f64) * t3491 * t12548;
    t45173
}
