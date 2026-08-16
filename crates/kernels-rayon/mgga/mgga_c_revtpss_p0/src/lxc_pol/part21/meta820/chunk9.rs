//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3036/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3036(t1086: f64, t15886: f64, t3151: f64, t4930: f64, t16543: f64, t3057: f64, t1087: f64, t1089: f64, t1090: f64, t12052: f64, t12122: f64, t12150: f64, t15609: f64, t16432: f64, t1689: f64, t3133: f64, t3259: f64, t3287: f64, t3299: f64, t3304: f64, t3316: f64, t342: f64, t43341: f64, t43413: f64, t43438: f64, t43456: f64, t4999: f64, t54064: f64, t54276: f64, t54365: f64, t54370: f64, t55165: f64, t55550: f64) -> (f64, f64) {
    let t55868 = t15886 * t1086;
    let t55880 = t4930 * t3151;
    let t55887 = t3057 * t16543;
    let t55894 = -0.19756347548806534796e1_f64 * t3287 * t55165 * t1089 - 0.65854491829355115987e0_f64 * t43341 * t54276 * t12052 - 0.39512695097613069591e1_f64 * t43456 * t16432 * t54370 - 0.79025390195226139182e1_f64 * t12122 * t55550 * t15609 + 0.19756347548806534796e1_f64 * t55868 * t1090 + 0.65854491829355115987e0_f64 * t43413 * t1689 + 0.19756347548806534796e1_f64 * t1087 * t4930 * t3133 * t1089 - 0.39512695097613069591e1_f64 * t12122 * t16432 * t54064 + 0.39512695097613069591e1_f64 * t3299 * t55880 * t3304 + 0.79025390195226139182e1_f64 * t43438 * t16432 * t54365 + 0.39512695097613069591e1_f64 * t55887 * t12150 - 0.19756347548806534796e1_f64 * t342 * t3316 * t3259 * t4999;
    (t55880, t55894)
}
