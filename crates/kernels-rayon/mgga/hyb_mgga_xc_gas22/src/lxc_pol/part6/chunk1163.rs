//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1163/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1163(t177: f64, t6358: f64, t2109: f64, t180: f64, t2111: f64, t746: f64, t6226: f64, t677: f64, t136: f64, t1815: f64, t2153: f64, t2986: f64, t765: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20467 = 1.0_f64 / t6358 / t177;
    let t20475 = 1.0_f64 / t6358 / t2109;
    let t20530 = t180 * t2111;
    let t20545 = 1.0_f64 / t6358 / t746;
    let t20560 = t677 * t6226;
    let t20563 = t136 * t1815 * t2153;
    let t20574 = t136 * t2986 * t765;
    (t20467, t20475, t20530, t20545, t20560, t20563, t20574)
}
