//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1800/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1800<F: Float>(t12756: F, t1285: F, t1287: F, t13129: F, t17192: F, t1774: F, t21439: F, t21579: F, t24770: F, t24986: F, t24989: F, t24998: F, t24999: F, t25002: F, t25005: F, t3755: F, t45738: F, t5332: F, t5463: F, t5464: F, t59681: F, t59749: F, t6622: F, t6695: F, t6731: F, t84487: F, t90162: F) -> F {
    let t91609 = -F::cast_from(0.26341796731742046395e1_f64) * t45738 * t90162 * t13129 + F::cast_from(0.79025390195226139184e1_f64) * t12756 * t84487 * t24998 + F::cast_from(0.52683593463484092788e1_f64) * t5463 * t5332 * t5464 * t24770 - F::cast_from(0.79025390195226139184e1_f64) * t21579 * t24999 - F::cast_from(0.79025390195226139183e1_f64) * t17192 * t24986 - F::cast_from(0.79025390195226139183e1_f64) * t17192 * t24989 + F::cast_from(0.79025390195226139183e1_f64) * t21439 * t6731 - F::cast_from(0.26341796731742046395e1_f64) * t3755 * t1774 * t24770 * t1287 - F::cast_from(0.15805078039045227836e2_f64) * t59749 * t25002 + F::cast_from(0.79025390195226139183e1_f64) * t59681 * t25005 + F::cast_from(0.39512695097613069592e1_f64) * t1285 * t6695 * t6622 * t1287;
    t91609
}
