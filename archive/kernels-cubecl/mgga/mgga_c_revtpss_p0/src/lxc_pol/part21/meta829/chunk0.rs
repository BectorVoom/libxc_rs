//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3092/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3092<F: Float>(t1210: F, t12607: F, t12628: F, t12629: F, t12658: F, t12666: F, t1277: F, t1294: F, t13166: F, t13182: F, t13183: F, t13184: F, t16750: F, t1774: F, t1775: F, t17992: F, t18005: F, t18062: F, t18065: F, t18097: F, t1828: F, t1829: F, t3576: F, t3585: F, t3732: F, t3739: F, t3791: F, t45515: F, t45522: F, t45535: F, t5220: F, t5225: F, t5246: F, t5417: F, t5423: F) -> F {
    let t56687 = F::cast_from(0.39512695097613069591e1_f64) * t18097 * t3576 - F::cast_from(0.19756347548806534796e1_f64) * t45535 * t1829 - F::cast_from(0.19756347548806534796e1_f64) * t45522 * t1775 - F::cast_from(0.19756347548806534796e1_f64) * t12658 * t5246 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t1277 * t16750 * t1294 + F::cast_from(0.19756347548806534796e1_f64) * t12666 * t5423 - F::cast_from(0.19756347548806534796e1_f64) * t18005 * t3791 + F::cast_from(0.39512695097613069591e1_f64) * t18005 * t3739 - F::cast_from(0.39512695097613069591e1_f64) * t5417 * t13184 + F::cast_from(0.39512695097613069591e1_f64) * t18062 * t3576 - F::cast_from(0.19756347548806534796e1_f64) * t18097 * t3585 - F::cast_from(0.39512695097613069591e1_f64) * t5225 * t13184 + F::cast_from(0.39512695097613069591e1_f64) * t18065 * t3739 + F::cast_from(0.19756347548806534796e1_f64) * t5220 * t12607 + F::cast_from(0.39512695097613069591e1_f64) * t1210 * t13182 * t1774 * t13183 + F::cast_from(0.39512695097613069591e1_f64) * t3732 * t17992 + F::cast_from(0.39512695097613069591e1_f64) * t12628 * t1277 * t1828 * t12629 - F::cast_from(0.65854491829355115987e0_f64) * t5225 * t13166 - F::cast_from(0.65854491829355115987e0_f64) * t45515 * t1775;
    t56687
}
