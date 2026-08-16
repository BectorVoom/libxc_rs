//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1654/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1654(t3475: f64, t426: f64, t3478: f64, t1179: f64, t12378: f64, t3488: f64, t3520: f64, t1161: f64, t1169: f64, t1180: f64, t1188: f64, t1189: f64, t12465: f64, t12470: f64, t12472: f64, t12473: f64, t12476: f64, t12481: f64, t12486: f64, t12488: f64, t12494: f64, t12548: f64, t12553: f64, t3447: f64, t3479: f64, t3480: f64, t3491: f64, t3498: f64, t3516: f64, t3523: f64, t3524: f64, t43753: f64, t43961: f64, t45057: f64, t45061: f64, t45064: f64, t45075: f64, t45080: f64, t45085: f64, t45103: f64, t45118: f64, t45134: f64, t45149: f64) -> f64 {
    let t45155 = t3475 * t3475;
    let t45157 = t426 / t45155;
    let t45158 = t3478 * t3478;
    let t45159 = 1.0_f64 / t45158;
    let t45163 = t12378 * t1179;
    let t45168 = t3488 * t3520;
    let t45173 = 0.5848223622634646207e0_f64 * t1180 * t43961 * t1188 + 0.11579025239058625248e4_f64 * t12470 * t45057 * t3479 - 0.70178683471615754484e1_f64 * t45061 * t3498 - 0.4155806185363551302e3_f64 * t45064 * t12488 + 0.6233709278045326953e3_f64 * t12553 * t43753 * t3523 + 0.14035736694323150897e2_f64 * t12481 * t12494 - 0.14035736694323150897e2_f64 * t12486 * t43753 * t1188 + 0.1929837539843104208e3_f64 * t45075 * t3480 + 4.0_f64 * t3447 * t12465 + 0.82761620670837440481e4_f64 * t45080 * t12473 - 0.24828486201251232145e5_f64 * t45085 * t45057 * t12472 + 1.0_f64 * t1161 * (t45103 + t45118 + t45134 + t45149) * t1169 + 0.19964560303604640732e6_f64 * t45157 * t45057 * t45159 + 0.23392894490538584828e1_f64 * t45163 * t1189 + 0.35089341735807877242e1_f64 * t12476 * t3516 + 0.10389515463408878255e3_f64 * t45168 * t3524 + 0.23392894490538584828e1_f64 * t3491 * t12548;
    t45173
}
