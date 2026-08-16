//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1344/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1344(t59688: f64, t59694: f64, t68444: f64, t68446: f64, t68448: f64, t68494: f64, t68498: f64, t76610: f64, t76614: f64, t76618: f64, t76622: f64, t76626: f64) -> f64 {
    let t76630 = 0.12361111111111111111e-1_f64 * t68444 + 0.13734567901234567901e-1_f64 * t68446 - 0.49444444444444444444e-1_f64 * t68448 + 0.24722222222222222222e-1_f64 * t68494 - 0.74166666666666666668e-1_f64 * t68498 - 0.24722222222222222222e-1_f64 * t76610 + 0.2225e0_f64 * t76614 - 0.33375e0_f64 * t76618 + 0.55625000000000000001e-1_f64 * t76622 + 0.74166666666666666668e-1_f64 * t76626 + 0.49444444444444444445e-1_f64 * t59688 - 0.24722222222222222222e-1_f64 * t59694;
    t76630
}
