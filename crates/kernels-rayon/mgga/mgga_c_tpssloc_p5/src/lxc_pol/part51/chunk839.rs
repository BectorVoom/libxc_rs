//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 839/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk839(t1528: f64, t2054: f64, t259: f64, t4147: f64, t4268: f64, t7067: f64, t7069: f64, t7087: f64, t7481: f64, t7486: f64, t7490: f64, t7815: f64, t7824: f64, t7830: f64, t7842: f64, t855: f64) -> f64 {
    let t7844 = -t7067 - 0.3289868133696452873e-1_f64 * t7481 - t7069 + 0.16449340668482264365e-1_f64 * t7486 - 0.16449340668482264365e-1_f64 * t7490 + t7815 * t259 + t7824 * t259 - t7087 * t1528 - t4147 * t2054 - t4268 * t2054 + 2.0_f64 * t855 * t7830 - t855 * t7842;
    t7844
}
