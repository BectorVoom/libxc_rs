//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 810/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk810(t2365: f64, t7778: f64, t5680: f64, t5688: f64, t959: f64, t325: f64, t883: f64, t900: f64, t6117: f64, t1710: f64, t2610: f64, t2033: f64) -> (f64, f64, f64, f64) {
    let t7779 = t2365 * t7778;
    let t7780 = t5680 * t7779;
    let t7782 = t5688 * t959;
    let t7784 = t883 * t325;
    let t7785 = t900 * t7784;
    let t7786 = t6117 * t7785;
    let t7788 = t2610 * t1710;
    let t7789 = t2365 * t7788;
    let t7790 = t2033 * t7789;
    (t7780, t7782, t7786, t7790)
}
