//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1625/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1625<F: Float>(t231: F, t87394: F, t40810: F, t40850: F, t51042: F, t51083: F, t51100: F, t51104: F, t62111: F, t62129: F, t76812: F, t76814: F, t76818: F, t76823: F, t76827: F, t76835: F, t76856: F, t76858: F, t825: F, t827: F, t828: F) -> (F, F) {
    let t87729 = t87394 * t231;
    let t87742 = -F::cast_from(0.17149607247227894789e-2_f64) * t76812 + F::cast_from(0.16006300097412701803e-1_f64) * t76814 + F::cast_from(0.28582678745379824648e-4_f64) * t76818 + F::cast_from(0.28582678745379824648e-4_f64) * t76823 + F::cast_from(0.17149607247227894789e-3_f64) * t76827 + F::cast_from(0.2168591159877823526e-3_f64) * t62111 - F::cast_from(0.64311027177104605458e-3_f64) * t825 * t827 * t828 * t87729 + F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t76835 + t40810 - F::cast_from(0.1829520101134271816e-3_f64) * t51042 + F::cast_from(0.91464571985215438873e-2_f64) * t62129 + F::cast_from(0.18071592998981862717e-5_f64) * t51083 + F::cast_from(0.34299214494455789577e-2_f64) * t76856 - t40850 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t76858 - F::cast_from(0.51384669507166276316e-2_f64) * t51100 + F::cast_from(0.15117061203111996148e0_f64) * t51104;
    (t87729, t87742)
}
