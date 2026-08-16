//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1149/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1149(t6652: f64, t794: f64, t6562: f64, t6547: f64, t6653: f64, t22723: f64, t6561: f64) -> (f64, f64, f64, f64) {
    let t23025 = t794 * t6652;
    let t23026 = t6562 * t23025;
    let t23028 = t6547 * t6653;
    let t23029 = 0.38381794893125283518e-1_f64 * t23028;
    let t23030 = t22723 * t6561;
    (t23025, t23026, t23029, t23030)
}
