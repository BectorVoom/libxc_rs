//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1362/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1362(t59688: f64, t59694: f64, t68444: f64, t68446: f64, t68448: f64, t68494: f64, t68498: f64, t76610: f64, t76614: f64, t76618: f64, t76622: f64, t76626: f64) -> f64 {
    let t77071 = 4.0_f64 / 9.0_f64 * t68444 + 40.0_f64 / 81.0_f64 * t68446 - 16.0_f64 / 9.0_f64 * t68448 + 8.0_f64 / 9.0_f64 * t68494 - 8.0_f64 / 3.0_f64 * t68498 - 8.0_f64 / 9.0_f64 * t76610 + 8.0_f64 * t76614 - 12.0_f64 * t76618 + 2.0_f64 * t76622 + 8.0_f64 / 3.0_f64 * t76626 + 16.0_f64 / 9.0_f64 * t59688 - 8.0_f64 / 9.0_f64 * t59694;
    t77071
}
