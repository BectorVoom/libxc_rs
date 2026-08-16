//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2560/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2560(t10480: f64, t13969: f64, t13986: f64, t3039: f64, t4599: f64, t49850: f64, t10870: f64, t4644: f64, t10875: f64, t48569: f64, t10937: f64, t13765: f64) -> (f64, f64, f64, f64, f64) {
    let t50255 = t10480 * t13969 * t13986;
    let t50258 = t3039 * t49850 * t4599;
    let t50262 = t4644 * t10870;
    let t50265 = t48569 * t10875;
    let t50272 = t10937 * t13765;
    (t50255, t50258, t50262, t50265, t50272)
}
