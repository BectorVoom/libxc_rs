//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1739/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1739<F: Float>(t1210: F, t1277: F, t13182: F, t1774: F, t17986: F, t17987: F, t18059: F, t1828: F, t1829: F, t20697: F, t20753: F, t21394: F, t21621: F, t24514: F, t24519: F, t24524: F, t24633: F, t24892: F, t24900: F, t25016: F, t25022: F, t5220: F, t5251: F, t5417: F, t6574: F, t6580: F, t6587: F, t6588: F, t6744: F, t6745: F, t72874: F, t84952: F) -> F {
    let t89930 = -F::cast_from(0.26341796731742046395e1_f64) * t84952 * t1829 + F::cast_from(0.15805078039045227836e2_f64) * t72874 * t6574 + F::cast_from(0.79025390195226139183e1_f64) * t5251 * t24900 + F::cast_from(0.15805078039045227836e2_f64) * t18059 * t24892 + F::cast_from(0.26341796731742046395e1_f64) * t1210 * t1277 * t24633 * t1828 - F::cast_from(0.39512695097613069592e1_f64) * t20697 * t6588 - F::cast_from(0.26341796731742046395e1_f64) * t5417 * t25016 + F::cast_from(0.79025390195226139183e1_f64) * t5220 * t24900 + F::cast_from(0.15805078039045227836e2_f64) * t1210 * t13182 * t24524 * t1774 + F::cast_from(0.79025390195226139183e1_f64) * t20697 * t6580 + F::cast_from(0.15805078039045227836e2_f64) * t21394 * t6580 - F::cast_from(0.39512695097613069592e1_f64) * t20753 * t6745 + F::cast_from(0.39512695097613069592e1_f64) * t1210 * t1277 * t6587 * t6744 - F::cast_from(0.15805078039045227836e2_f64) * t5251 * t24519 - F::cast_from(0.15805078039045227836e2_f64) * t17986 * t17987 * t24514 - F::cast_from(0.26341796731742046395e1_f64) * t5251 * t25022 - F::cast_from(0.39512695097613069592e1_f64) * t21621 * t6588;
    t89930
}
