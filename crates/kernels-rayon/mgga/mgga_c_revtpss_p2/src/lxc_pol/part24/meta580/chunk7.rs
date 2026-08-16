//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1800/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1800(t12756: f64, t1285: f64, t1287: f64, t13129: f64, t17192: f64, t1774: f64, t21439: f64, t21579: f64, t24770: f64, t24986: f64, t24989: f64, t24998: f64, t24999: f64, t25002: f64, t25005: f64, t3755: f64, t45738: f64, t5332: f64, t5463: f64, t5464: f64, t59681: f64, t59749: f64, t6622: f64, t6695: f64, t6731: f64, t84487: f64, t90162: f64) -> f64 {
    let t91609 = -0.26341796731742046395e1_f64 * t45738 * t90162 * t13129 + 0.79025390195226139184e1_f64 * t12756 * t84487 * t24998 + 0.52683593463484092788e1_f64 * t5463 * t5332 * t5464 * t24770 - 0.79025390195226139184e1_f64 * t21579 * t24999 - 0.79025390195226139183e1_f64 * t17192 * t24986 - 0.79025390195226139183e1_f64 * t17192 * t24989 + 0.79025390195226139183e1_f64 * t21439 * t6731 - 0.26341796731742046395e1_f64 * t3755 * t1774 * t24770 * t1287 - 0.15805078039045227836e2_f64 * t59749 * t25002 + 0.79025390195226139183e1_f64 * t59681 * t25005 + 0.39512695097613069592e1_f64 * t1285 * t6695 * t6622 * t1287;
    t91609
}
