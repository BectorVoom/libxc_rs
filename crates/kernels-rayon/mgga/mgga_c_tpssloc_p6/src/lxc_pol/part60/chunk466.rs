//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 466/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk466(t193: f64, t202: f64, t2378: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2665: f64, t2752: f64, t5527: f64, t5544: f64, t5596: f64, t5599: f64, t5660: f64, t5664: f64, t766: f64, t870: f64) -> f64 {
    let t5668 = -t193 * t202 * t2752 * t5664 + t193 * t202 * t5660 * t870 + 6.0_f64 * t193 * t2378 * t5527 + 3.0_f64 * t193 * t5544 * t766 - t2423 - t2426 - t2486 + t2518 - t2530 - t2537 + t2665 - t5596 + t5599;
    t5668
}
