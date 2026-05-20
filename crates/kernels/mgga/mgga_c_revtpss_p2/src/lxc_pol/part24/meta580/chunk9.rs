//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1802/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1802<F: Float>(t1234: F, t1280: F, t1285: F, t1287: F, t17183: F, t17307: F, t1770: F, t1774: F, t1794: F, t1818: F, t1825: F, t24698: F, t24864: F, t24912: F, t24915: F, t24922: F, t24994: F, t25009: F, t3670: F, t5436: F, t59498: F, t83108: F, t84429: F, t89808: F, t89960: F) -> F {
    let t91671 = F::cast_from(0.26341796731742046395e1_f64) * t1770 * t24915 + F::cast_from(0.26341796731742046395e1_f64) * t1285 * t24864 * t1794 * t1287 - F::cast_from(0.15805078039045227836e2_f64) * t59498 * t24912 - F::cast_from(0.26341796731742046395e1_f64) * t83108 * t1818 + F::cast_from(0.26341796731742046395e1_f64) * t5436 * t25009 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t1280 * t89960 + F::cast_from(0.15805078039045227836e2_f64) * t17307 * t24922 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t1280 * t89808 - F::cast_from(0.26341796731742046395e1_f64) * t1234 * t84429 * t1774 - F::cast_from(0.79025390195226139183e1_f64) * t17183 * t24994 + F::cast_from(0.26341796731742046395e1_f64) * t24698 * t1825;
    t91671
}
