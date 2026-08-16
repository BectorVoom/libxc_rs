//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 690/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk690(t1894: f64, t236: f64, t776: f64, t6591: f64, t2229: f64, t61: f64, t1891: f64, t133: f64, t119: f64, t212: f64, t1895: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6593 = t1894 * t236 * t776;
    let t6594 = t6591 * t6593;
    let t6597 = 1.0_f64 / t61 / t2229;
    let t6598 = t6597 * t1891;
    let t6599 = t6598 * t133;
    let t6600 = t119 * t212;
    let t6601 = t6600 * t1895;
    let t6602 = t6599 * t6601;
    let t6603 = 0.33643963411783659045e-4_f64 * t6602;
    let t6604 = t213 * t225;
    (t6593, t6594, t6597, t6598, t6599, t6600, t6601, t6602, t6603, t6604)
}
