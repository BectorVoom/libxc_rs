//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 829/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk829(t22990: f64, t23000: f64, t23002: f64, t23006: f64, t23022: f64, t23026: f64, t23028: f64, t23038: f64, t24246: f64, t24250: f64, t24251: f64, t24256: f64, t2617: f64, t7102: f64, t812: f64) -> f64 {
    let t24260 = 0.6579736267392905746e-1_f64 * t22990 + 0.3289868133696452873e-1_f64 * t23000 + 0.76763589786250567036e-1_f64 * t23002 - 0.16449340668482264365e-1_f64 * t23006 + t24246 + 0.16449340668482264365e-1_f64 * t23022 - 0.16449340668482264365e-1_f64 * t23026 - 0.76763589786250567036e-1_f64 * t23028 + t24250 - t812 * t24251 - 2.0_f64 * t2617 * t7102 + 2.0_f64 * t812 * t24256 + 0.9869604401089358619e-1_f64 * t23038;
    t24260
}
