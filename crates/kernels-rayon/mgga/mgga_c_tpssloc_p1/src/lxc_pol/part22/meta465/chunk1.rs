//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1848/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1848(t221: f64, t5196: f64, t6347: f64, t12188: f64, t12194: f64, t12196: f64, t12215: f64, t12236: f64, t1315: f64, t16078: f64, t16108: f64, t16119: f64, t19768: f64, t19776: f64, t19779: f64, t19791: f64, t20576: f64, t20582: f64, t5195: f64) -> (f64, f64) {
    let t20586 = t221 * t5196 * t6347;
    let t20594 = -0.16666666666666666666e-2_f64 * t1315 * t20576 - t12188 - 0.74999999999999999997e-2_f64 * t19768 + 0.24999999999999999999e-2_f64 * t19776 - t12194 + t12196 - 0.19999999999999999999e-1_f64 * t12215 * t20582 + 0.14999999999999999999e-1_f64 * t5195 * t20586 - 0.34999999999999999998e-1_f64 * t19779 + 0.11666666666666666666e-1_f64 * t19791 - 0.38888888888888888888e-1_f64 * t16078 - t12236 - 0.15833333333333333333e-1_f64 * t16108 + 0.49999999999999999998e-2_f64 * t16119;
    (t20586, t20594)
}
