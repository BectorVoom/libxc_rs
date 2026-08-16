//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1422/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1422(t33250: f64, t6914: f64, t115614: f64, t1842: f64, t1992: f64, t22635: f64, t113934: f64, t115292: f64, t115294: f64, t120180: f64, t120184: f64, t120196: f64, t122102: f64, t122107: f64, t122110: f64, t2092: f64, t24082: f64, t7750: f64, t90732: f64) -> f64 {
    let t122112 = t6914 * t33250;
    let t122117 = t1992 * t22635 * t115614 * t1842;
    let t122119 = t120180 + t120184 - t90732 * t2092 - 0.38381794893125283518e-1_f64 * t122102 - t24082 * t7750 + t113934 + 0.19190897446562641759e-1_f64 * t115292 + 0.16449340668482264365e-1_f64 * t122107 + 0.16449340668482264365e-1_f64 * t122110 - 0.38381794893125283518e-1_f64 * t122112 + 0.19190897446562641759e-1_f64 * t115294 - t120196 + 0.16449340668482264365e-1_f64 * t122117;
    t122119
}
