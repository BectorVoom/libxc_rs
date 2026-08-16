//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1792/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1792(t2332: f64, t81442: f64, t22470: f64, t2358: f64, t63: f64, t9365: f64, t2752: f64, t606: f64, t23020: f64, t6562: f64, t794: f64, t22641: f64, t9523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81443 = t81442 * t2332;
    let t81445 = t22470 * t2358;
    let t81446 = t63 * t9365;
    let t81547 = t2752 * t606;
    let t81571 = t6562 * t794 * t23020;
    let t81573 = t22641 * t9523;
    (t81443, t81445, t81446, t81547, t81571, t81573)
}
