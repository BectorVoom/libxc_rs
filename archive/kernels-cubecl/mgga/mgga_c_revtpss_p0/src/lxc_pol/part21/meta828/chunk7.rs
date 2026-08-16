//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3091/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3091<F: Float>(t3566: F, t5412: F, t3568: F, t5245: F, t1210: F, t1211: F, t12599: F, t12600: F, t12628: F, t12633: F, t12647: F, t12654: F, t12658: F, t12673: F, t1274: F, t1277: F, t13165: F, t13174: F, t1774: F, t17973: F, t17987: F, t17995: F, t17999: F, t18030: F, t18054: F, t18059: F, t18070: F, t18087: F, t18114: F, t3556: F, t3569: F, t3576: F, t3737: F, t3739: F, t3790: F, t3791: F, t45427: F, t45449: F, t5220: F, t5237: F, t5429: F, t5497: F, t5498: F) -> (F, F) {
    let t56607 = t3566 * t5412;
    let t56620 = t5245 * t3568;
    let t56642 = F::cast_from(0.79025390195226139182e1_f64) * t17973 * t17987 * t12599 + F::cast_from(0.19756347548806534796e1_f64) * t3556 * t17999 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1277 * t1774 * t13165 + F::cast_from(0.39512695097613069591e1_f64) * t18054 * t3739 + F::cast_from(0.39512695097613069591e1_f64) * t56607 * t3569 - F::cast_from(0.39512695097613069591e1_f64) * t18059 * t12600 - F::cast_from(0.19756347548806534796e1_f64) * t18054 * t3791 + F::cast_from(0.39512695097613069591e1_f64) * t18087 * t3739 - F::cast_from(0.19756347548806534796e1_f64) * t12654 * t5498 - F::cast_from(0.39512695097613069591e1_f64) * t5220 * t13174 - F::cast_from(0.11853808529283920877e2_f64) * t12628 * t1211 * t56620 + F::cast_from(0.39512695097613069591e1_f64) * t18114 * t3576 - F::cast_from(0.11853808529283920877e2_f64) * t45449 * t18030 + F::cast_from(0.39512695097613069591e1_f64) * t17995 * t12647 + F::cast_from(0.39512695097613069591e1_f64) * t12673 * t5429 + F::cast_from(0.19756347548806534796e1_f64) * t12658 * t5237 + F::cast_from(0.79025390195226139182e1_f64) * t12633 * t18070 + F::cast_from(0.39512695097613069591e1_f64) * t1274 * t3737 * t5497 * t3790 - F::cast_from(0.11853808529283920877e2_f64) * t45427 * t18030;
    (t56620, t56642)
}
