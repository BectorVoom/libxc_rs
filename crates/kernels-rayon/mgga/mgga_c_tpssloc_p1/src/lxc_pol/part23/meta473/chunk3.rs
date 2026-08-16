//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1416/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1416(t63332: f64, t63334: f64, t63361: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t77989: f64, t77992: f64, t77995: f64, t78057: f64, t43820: f64, t50834: f64, t71154: f64, t71156: f64, t77998: f64, t78002: f64, t78005: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64) -> (f64, f64) {
    let t78064 = -16.0_f64 / 27.0_f64 * t63332 + 8.0_f64 / 9.0_f64 * t63334 + 8.0_f64 / 9.0_f64 * t71142 - 8.0_f64 / 3.0_f64 * t71144 + 16.0_f64 / 9.0_f64 * t63361 - 8.0_f64 * t78057 - 40.0_f64 / 81.0_f64 * t71146 + 8.0_f64 * t77989 + t77992 / 3.0_f64 - 80.0_f64 / 81.0_f64 * t77995 - 8.0_f64 / 3.0_f64 * t71152;
    let t78076 = -4.0_f64 / 9.0_f64 * t71154 + 2.0_f64 * t77998 + 16.0_f64 / 9.0_f64 * t71156 + 40.0_f64 / 9.0_f64 * t78002 - 8.0_f64 / 9.0_f64 * t78033 - 112.0_f64 / 81.0_f64 * t50834 + t43820 + 20.0_f64 / 9.0_f64 * t78037 - 8.0_f64 * t78041 + 12.0_f64 * t78045 + 8.0_f64 / 3.0_f64 * t78049 - 2.0_f64 / 3.0_f64 * t78005;
    (t78064, t78076)
}
