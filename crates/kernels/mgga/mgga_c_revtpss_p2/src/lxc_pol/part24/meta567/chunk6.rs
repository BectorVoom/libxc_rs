//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1738/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1738<F: Float>(t459: F, t89857: F, t89881: F, t1211: F, t12628: F, t1274: F, t1277: F, t13182: F, t1770: F, t1828: F, t1829: F, t20753: F, t20756: F, t21394: F, t24509: F, t24519: F, t24525: F, t24616: F, t24866: F, t25022: F, t3567: F, t3737: F, t495: F, t5220: F, t5225: F, t5417: F, t6573: F, t6574: F, t6588: F, t6702: F, t6703: F, t6744: F, t6745: F, t72802: F, t82147: F, t89808: F) -> (F, F) {
    let t89883 = (t89857 + t89881) * t459;
    let t89888 = -F::cast_from(0.79025390195226139183e1_f64) * t21394 * t6588 - F::cast_from(0.26341796731742046395e1_f64) * t5220 * t25022 + F::cast_from(0.26341796731742046395e1_f64) * t1770 * t24866 + F::cast_from(0.15805078039045227836e2_f64) * t12628 * t1277 * t24616 * t1828 - F::cast_from(0.26341796731742046395e1_f64) * t82147 * t1829 + F::cast_from(0.15805078039045227836e2_f64) * t5225 * t24509 - F::cast_from(0.15805078039045227836e2_f64) * t5220 * t24519 - F::cast_from(0.79025390195226139183e1_f64) * t20756 * t6745 + F::cast_from(0.15805078039045227836e2_f64) * t3567 * t3737 * t6573 * t6702 + F::cast_from(0.15805078039045227836e2_f64) * t5417 * t24509 + F::cast_from(0.39512695097613069591e1_f64) * t3567 * t1211 * t89808 - F::cast_from(0.15805078039045227836e2_f64) * t5417 * t24525 + F::cast_from(0.79025390195226139183e1_f64) * t20753 * t6703 - F::cast_from(0.23707617058567841754e2_f64) * t1274 * t13182 * t6702 * t6744 + F::cast_from(0.15805078039045227836e2_f64) * t20756 * t6703 + F::cast_from(0.65854491829355115987e0_f64) * t89883 * t495 + F::cast_from(0.79025390195226139183e1_f64) * t72802 * t6574;
    (t89883, t89888)
}
