//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1391/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1391(t33429: f64, t6547: f64, t7841: f64, t857: f64, t22986: f64, t23270: f64, t776: f64, t114900: f64, t118859: f64, t118871: f64, t118874: f64, t2054: f64, t23278: f64, t25168: f64, t25188: f64, t25232: f64, t26700: f64, t26728: f64, t2713: f64, t33433: f64, t6663: f64, t7107: f64, t7830: f64, t7842: f64, t86988: f64) -> f64 {
    let t121629 = t6547 * t33429;
    let t121634 = t857 * t7841;
    let t121637 = t22986 * t23270 * t121634 * t776;
    let t121643 = -6.0_f64 * t25168 * t26728 * t25232 - t25188 * t7107 - t23278 * t7842 - 0.19190897446562641759e-1_f64 * t121629 + t118859 + 2.0_f64 * t23278 * t7830 - t86988 * t2054 - t118871 + 0.16449340668482264365e-1_f64 * t121637 + 2.0_f64 * t2713 * t33433 - t118874 + 0.38381794893125283518e-1_f64 * t114900 - t26700 * t6663;
    t121643
}
