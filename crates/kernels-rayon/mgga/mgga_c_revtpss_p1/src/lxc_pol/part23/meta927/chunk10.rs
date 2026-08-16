//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3019/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3019(t24007: f64, t3153: f64, t23992: f64, t23837: f64, t1071: f64, t23640: f64, t12078: f64, t12079: f64, t12116: f64, t12122: f64, t12127: f64, t12149: f64, t19447: f64, t19483: f64, t19579: f64, t19593: f64, t23997: f64, t24079: f64, t24112: f64, t43438: f64, t43456: f64, t43574: f64, t4743: f64, t4976: f64, t4983: f64, t4998: f64, t55887: f64, t6389: f64, t73: f64) -> (f64, f64, f64) {
    let t80312 = t24007 * t3153;
    let t80319 = t23992 * t3153;
    let t80330 = t23837 * t3153;
    let t80341 = t1071 * t23640;
    let t80349 = -0.39512695097613069592e1_f64 * t43456 * t80312 * t19579 + 0.39512695097613069592e1_f64 * t12127 * t19593 * t19483 - 0.39512695097613069592e1_f64 * t12122 * t80319 * t4983 + 0.19756347548806534796e1_f64 * t12127 * t80319 * t4998 + 0.39512695097613069592e1_f64 * t12149 * t23997 * t73 * t4976 + 0.79025390195226139182e1_f64 * t43438 * t80330 * t4983 - 0.39512695097613069591e1_f64 * t43456 * t80330 * t4998 + 0.79025390195226139182e1_f64 * t55887 * t19447 + 0.19756347548806534796e1_f64 * t4743 * t6389 - 0.39512695097613069591e1_f64 * t12078 * t80341 * t12079 - 0.39512695097613069591e1_f64 * t43574 * t24079 + 0.39512695097613069591e1_f64 * t12116 * t24112;
    (t80312, t80341, t80349)
}
