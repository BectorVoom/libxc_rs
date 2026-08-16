//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1274/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1274(t11605: f64, t225: f64, t11545: f64, t461: f64, t491: f64, t1009: f64, t460: f64, t27495: f64, t1193: f64, t24811: f64, t3545: f64, t7372: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85674 = t225 * t11605;
    let t85754 = t11545 * t461;
    let t85755 = t85754 * t491;
    let t85821 = t460 * t1009;
    let t85822 = t85821 * t27495;
    let t85853 = t24811 * t1193;
    let t85909 = t85754 * t225;
    let t85917 = t7372 * t3545;
    (t85674, t85755, t85822, t85853, t85909, t85917)
}
