//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1425/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1425(t1099: f64, t1118: f64, t78147: f64, t78162: f64, t78177: f64, t78191: f64, t44075: f64, t44077: f64, t78129: f64, t63332: f64, t63334: f64, t63361: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t77989: f64, t77992: f64, t77995: f64, t78057: f64) -> (f64, f64, f64) {
    let t78196 = 1.0_f64 * t1099 * (t78147 + t78162 + t78177 + t78191) * t1118;
    let t78199 = 0.24955700379505800916e5_f64 * t44075 * t78129 * t44077;
    let t78211 = -0.16481481481481481482e-1_f64 * t63332 + 0.24722222222222222222e-1_f64 * t63334 + 0.24722222222222222222e-1_f64 * t71142 - 0.74166666666666666668e-1_f64 * t71144 + 0.49444444444444444445e-1_f64 * t63361 - 0.22249999999999999999e0_f64 * t78057 - 0.13734567901234567901e-1_f64 * t71146 + 0.2225e0_f64 * t77989 + 0.92708333333333333333e-2_f64 * t77992 - 0.27469135802469135803e-1_f64 * t77995 - 0.74166666666666666668e-1_f64 * t71152;
    (t78196, t78199, t78211)
}
