//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3036/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3036<F: Float>(t1086: F, t15886: F, t3151: F, t4930: F, t16543: F, t3057: F, t1087: F, t1089: F, t1090: F, t12052: F, t12122: F, t12150: F, t15609: F, t16432: F, t1689: F, t3133: F, t3259: F, t3287: F, t3299: F, t3304: F, t3316: F, t342: F, t43341: F, t43413: F, t43438: F, t43456: F, t4999: F, t54064: F, t54276: F, t54365: F, t54370: F, t55165: F, t55550: F) -> (F, F) {
    let t55868 = t15886 * t1086;
    let t55880 = t4930 * t3151;
    let t55887 = t3057 * t16543;
    let t55894 = -F::cast_from(0.19756347548806534796e1_f64) * t3287 * t55165 * t1089 - F::cast_from(0.65854491829355115987e0_f64) * t43341 * t54276 * t12052 - F::cast_from(0.39512695097613069591e1_f64) * t43456 * t16432 * t54370 - F::cast_from(0.79025390195226139182e1_f64) * t12122 * t55550 * t15609 + F::cast_from(0.19756347548806534796e1_f64) * t55868 * t1090 + F::cast_from(0.65854491829355115987e0_f64) * t43413 * t1689 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t4930 * t3133 * t1089 - F::cast_from(0.39512695097613069591e1_f64) * t12122 * t16432 * t54064 + F::cast_from(0.39512695097613069591e1_f64) * t3299 * t55880 * t3304 + F::cast_from(0.79025390195226139182e1_f64) * t43438 * t16432 * t54365 + F::cast_from(0.39512695097613069591e1_f64) * t55887 * t12150 - F::cast_from(0.19756347548806534796e1_f64) * t342 * t3316 * t3259 * t4999;
    (t55880, t55894)
}
