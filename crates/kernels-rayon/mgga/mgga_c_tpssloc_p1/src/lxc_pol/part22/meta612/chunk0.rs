//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2140/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2140(t11282: f64, t1687: f64, t1682: f64, t3357: f64, t1694: f64, t3401: f64, t11420: f64, t3312: f64, t4737: f64, t11419: f64, t1675: f64, t50826: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51376 = t1687 * t11282;
    let t51382 = t3357 * t1682;
    let t51389 = t3401 * t1694;
    let t51392 = t11420 * t1682;
    let t51402 = t4737 * t3312;
    let t51427 = t1675 * t11419;
    let t51550 = 0.23744444444444444444e-1_f64 * t50826;
    (t51376, t51382, t51389, t51392, t51402, t51427, t51550)
}
