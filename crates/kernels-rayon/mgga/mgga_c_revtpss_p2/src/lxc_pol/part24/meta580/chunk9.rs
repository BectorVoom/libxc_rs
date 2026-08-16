//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1802/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1802(t1234: f64, t1280: f64, t1285: f64, t1287: f64, t17183: f64, t17307: f64, t1770: f64, t1774: f64, t1794: f64, t1818: f64, t1825: f64, t24698: f64, t24864: f64, t24912: f64, t24915: f64, t24922: f64, t24994: f64, t25009: f64, t3670: f64, t5436: f64, t59498: f64, t83108: f64, t84429: f64, t89808: f64, t89960: f64) -> f64 {
    let t91671 = 0.26341796731742046395e1_f64 * t1770 * t24915 + 0.26341796731742046395e1_f64 * t1285 * t24864 * t1794 * t1287 - 0.15805078039045227836e2_f64 * t59498 * t24912 - 0.26341796731742046395e1_f64 * t83108 * t1818 + 0.26341796731742046395e1_f64 * t5436 * t25009 - 0.65854491829355115987e0_f64 * t1234 * t1280 * t89960 + 0.15805078039045227836e2_f64 * t17307 * t24922 + 0.39512695097613069591e1_f64 * t3670 * t1280 * t89808 - 0.26341796731742046395e1_f64 * t1234 * t84429 * t1774 - 0.79025390195226139183e1_f64 * t17183 * t24994 + 0.26341796731742046395e1_f64 * t24698 * t1825;
    t91671
}
