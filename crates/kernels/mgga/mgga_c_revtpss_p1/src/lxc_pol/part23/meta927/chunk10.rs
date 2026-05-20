//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3019/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3019<F: Float>(t24007: F, t3153: F, t23992: F, t23837: F, t1071: F, t23640: F, t12078: F, t12079: F, t12116: F, t12122: F, t12127: F, t12149: F, t19447: F, t19483: F, t19579: F, t19593: F, t23997: F, t24079: F, t24112: F, t43438: F, t43456: F, t43574: F, t4743: F, t4976: F, t4983: F, t4998: F, t55887: F, t6389: F, t73: F) -> (F, F, F) {
    let t80312 = t24007 * t3153;
    let t80319 = t23992 * t3153;
    let t80330 = t23837 * t3153;
    let t80341 = t1071 * t23640;
    let t80349 = -F::cast_from(0.39512695097613069592e1_f64) * t43456 * t80312 * t19579 + F::cast_from(0.39512695097613069592e1_f64) * t12127 * t19593 * t19483 - F::cast_from(0.39512695097613069592e1_f64) * t12122 * t80319 * t4983 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t80319 * t4998 + F::cast_from(0.39512695097613069592e1_f64) * t12149 * t23997 * t73 * t4976 + F::cast_from(0.79025390195226139182e1_f64) * t43438 * t80330 * t4983 - F::cast_from(0.39512695097613069591e1_f64) * t43456 * t80330 * t4998 + F::cast_from(0.79025390195226139182e1_f64) * t55887 * t19447 + F::cast_from(0.19756347548806534796e1_f64) * t4743 * t6389 - F::cast_from(0.39512695097613069591e1_f64) * t12078 * t80341 * t12079 - F::cast_from(0.39512695097613069591e1_f64) * t43574 * t24079 + F::cast_from(0.39512695097613069591e1_f64) * t12116 * t24112;
    (t80312, t80341, t80349)
}
