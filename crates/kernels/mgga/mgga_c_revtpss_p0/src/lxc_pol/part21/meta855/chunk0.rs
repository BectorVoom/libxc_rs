//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3235/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3235<F: Float>(t1210: F, t1214: F, t12600: F, t12603: F, t12606: F, t12666: F, t12690: F, t12696: F, t1274: F, t1277: F, t13177: F, t1775: F, t17963: F, t17975: F, t17986: F, t17987: F, t17992: F, t17995: F, t18037: F, t18047: F, t18065: F, t1813: F, t1829: F, t225: F, t3556: F, t3561: F, t3569: F, t3576: F, t3791: F, t45433: F, t45545: F, t45575: F, t460: F, t494: F, t5231: F, t5237: F, t5246: F, t5417: F, t5498: F, t56707: F, t59453: F, t59464: F, t59510: F, t59544: F, t59579: F, t59611: F, t59649: F, t59689: F, t59724: F, t59762: F, t59797: F, t59833: F, t59877: F, t59916: F, t59951: F, t59983: F, t60022: F, t60058: F) -> F {
    let t60068 = F::cast_from(0.39512695097613069591e1_f64) * t3561 * t17992 - F::cast_from(0.19756347548806534796e1_f64) * t12666 * t5246 + F::cast_from(0.39512695097613069591e1_f64) * t5417 * t12696 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t1277 * t17963 * t1214 + F::cast_from(0.65854491829355115987e0_f64) * t12690 * t1813 - F::cast_from(0.19756347548806534796e1_f64) * t18065 * t3791 - F::cast_from(0.39512695097613069591e1_f64) * t12603 * t5498 - F::cast_from(0.39512695097613069591e1_f64) * t3556 * t18047 - F::cast_from(0.79025390195226139182e1_f64) * t56707 * t17975 + F::cast_from(0.39512695097613069591e1_f64) * t13177 * t5237 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t59453 * t225 * t494 - F::cast_from(0.39512695097613069591e1_f64) * t17995 * t12600 + F::cast_from(0.39512695097613069591e1_f64) * t18037 * t3576 - F::cast_from(0.19756347548806534796e1_f64) * t45545 * t1775 + F::cast_from(0.39512695097613069591e1_f64) * t59464 * t3569 - F::cast_from(0.65854491829355115987e0_f64) * t45575 * t1829 - F::cast_from(0.39512695097613069591e1_f64) * t17986 * t17987 * t12606 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1277 * (t59510 + t59544 + t59579 + t59611 + t59649 + t59689 + t59724 + t59762 + t59797 + t59833 + t59877 + t59916 + t59951 + t59983 + t60022 + t60058) + F::cast_from(0.79025390195226139182e1_f64) * t45433 * t5231;
    t60068
}
