//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2537/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2537(t71371: f64, t71389: f64, t1107: f64, t63332: f64, t63334: f64, t63336: f64, t63886: f64, t63888: f64, t63893: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64) -> (f64, f64, f64) {
    let t71390 = t71371 + t71389;
    let t71391 = t1107 * t71390;
    let t71396 = 0.10064166666666666667e1_f64 * t71124 - 0.26837777777777777777e0_f64 * t63332 + 0.40256666666666666668e0_f64 * t63334 - 0.30192500000000000001e0_f64 * t63336 - 0.36231e1_f64 * t71130 - 0.16557e0_f64 * t63886 - 0.91983333333333333334e-1_f64 * t63888 + 0.5519e0_f64 * t63893 + 0.16504875e0_f64 * t71391 + 0.40256666666666666666e1_f64 * t71135 - 0.20128333333333333333e0_f64 * t71140 + 0.20128333333333333333e0_f64 * t71142;
    (t71390, t71391, t71396)
}
