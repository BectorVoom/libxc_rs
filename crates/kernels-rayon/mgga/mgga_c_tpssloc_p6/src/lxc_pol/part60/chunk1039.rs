//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1039/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1039(t102917: f64, t114225: f64, t114264: f64, t122331: f64, t127354: f64, t127355: f64, t127422: f64, t127423: f64, t127427: f64, t128797: f64, t128805: f64, t128809: f64, t128816: f64, t20029: f64, t2016: f64, t26224: f64, t26477: f64, t26989: f64, t27009: f64, t28110: f64, t7750: f64, t7937: f64, t8637: f64) -> f64 {
    let t128818 = t114225 - 2.0_f64 * t20029 * t8637 - 2.0_f64 * t27009 * t7750 + t127354 + t127355 + 0.16449340668482264365e-1_f64 * t122331 - 0.16449340668482264365e-1_f64 * t128797 - 2.0_f64 * t26477 * t7937 - 2.0_f64 * t102917 * t2016 - 0.6579736267392905746e-1_f64 * t128805 + t114264 - t127422 + 0.3289868133696452873e-1_f64 * t128809 + t127423 - 6.0_f64 * t26224 * t26989 * t28110 - 0.49348022005446793095e-1_f64 * t128816 + t127427;
    t128818
}
