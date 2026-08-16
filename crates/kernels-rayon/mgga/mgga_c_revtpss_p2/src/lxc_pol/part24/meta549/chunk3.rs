//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1625/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1625(t231: f64, t87394: f64, t40810: f64, t40850: f64, t51042: f64, t51083: f64, t51100: f64, t51104: f64, t62111: f64, t62129: f64, t76812: f64, t76814: f64, t76818: f64, t76823: f64, t76827: f64, t76835: f64, t76856: f64, t76858: f64, t825: f64, t827: f64, t828: f64) -> (f64, f64) {
    let t87729 = t87394 * t231;
    let t87742 = -0.17149607247227894789e-2_f64 * t76812 + 0.16006300097412701803e-1_f64 * t76814 + 0.28582678745379824648e-4_f64 * t76818 + 0.28582678745379824648e-4_f64 * t76823 + 0.17149607247227894789e-3_f64 * t76827 + 0.2168591159877823526e-3_f64 * t62111 - 0.64311027177104605458e-3_f64 * t825 * t827 * t828 * t87729 + 7.0_f64 / 3.0_f64 * t76835 + t40810 - 0.1829520101134271816e-3_f64 * t51042 + 0.91464571985215438873e-2_f64 * t62129 + 0.18071592998981862717e-5_f64 * t51083 + 0.34299214494455789577e-2_f64 * t76856 - t40850 + 7.0_f64 / 36.0_f64 * t76858 - 0.51384669507166276316e-2_f64 * t51100 + 0.15117061203111996148e0_f64 * t51104;
    (t87729, t87742)
}
