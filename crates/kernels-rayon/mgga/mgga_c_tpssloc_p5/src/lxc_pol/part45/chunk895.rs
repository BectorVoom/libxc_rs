//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 895/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk895(t26989: f64, t6962: f64, t6883: f64, t8612: f64, t1386: f64, t2016: f64, t24082: f64, t26224: f64, t31147: f64, t31646: f64, t31649: f64, t31651: f64, t31653: f64, t3758: f64, t3882: f64, t8627: f64, t8637: f64) -> (f64, f64) {
    let t31655 = t26989 * t6962;
    let t31662 = t6883 * t8612;
    let t31663 = 0.19190897446562641759e-1_f64 * t31662;
    let t31666 = -0.16449340668482264365e-1_f64 * t31646 - t31147 + t31649 - 0.82246703342411321825e-2_f64 * t31651 - t31653 * t1386 - 6.0_f64 * t26224 * t31655 + 2.0_f64 * t3758 * t8627 + 2.0_f64 * t3882 * t8627 - t31663 - t24082 * t2016 - t3758 * t8637;
    (t31655, t31666)
}
