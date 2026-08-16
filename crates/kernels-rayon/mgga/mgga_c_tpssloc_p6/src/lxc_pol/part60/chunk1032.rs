//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1032/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1032(t128616: f64, t128625: f64, t28209: f64, t31611: f64, t6888: f64, t122166: f64, t1985: f64, t7700: f64, t113934: f64, t115306: f64, t122102: f64, t122112: f64, t122121: f64, t127166: f64, t127169: f64, t127173: f64, t127176: f64, t127180: f64, t127183: f64, t128604: f64, t33323: f64, t539: f64, t568: f64, t6361: f64, t8617: f64, t97626: f64) -> (f64, f64) {
    let t128626 = t128616 + t128625;
    let t128630 = t6888 * t31611 * t28209;
    let t128633 = t1985 * t122166 * t7700;
    let t128639 = t127166 + t6361 * t8617 * t568 + 0.3289868133696452873e-1_f64 * t128604 + t127169 - 0.76763589786250567036e-1_f64 * t122102 + t539 * t128626 * t568 + t127173 + t113934 + t127176 - 0.16449340668482264365e-1_f64 * t128630 - 0.16449340668482264365e-1_f64 * t128633 + t127180 - 0.76763589786250567036e-1_f64 * t122112 - 12.0_f64 * t97626 * t33323 - t115306 + 0.82246703342411321824e-2_f64 * t122121 + t127183;
    (t128626, t128639)
}
