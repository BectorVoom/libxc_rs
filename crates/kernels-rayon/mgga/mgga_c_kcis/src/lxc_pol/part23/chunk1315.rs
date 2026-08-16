//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1315/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1315(t187: f64, t97635: f64, t97637: f64, t97638: f64, t97641: f64, t97643: f64, t97645: f64, t97647: f64, t97650: f64, t97652: f64, t97654: f64, t97657: f64, t97845: f64, t97852: f64, t97854: f64, t97856: f64, t97862: f64, t97870: f64, t97875: f64, t97880: f64, t98957: f64, t99733: f64) -> f64 {
    let t99737 = t97635 + t97637 - t97638 + t97641 + t97643 + t97645 - t97647 - t97650 - t97652 + t97654 + t97657 + t187 * (t97845 + t97880 + t98957 + t99733) - t97852 - t97854 + t97856 + t97862 + t97870 + t97875;
    t99737
}
