//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 701/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk701(t5: f64, t6504: f64, t67: f64, t1864: f64, t641: f64, t71: f64, t1863: f64, t1860: f64, t1865: f64, t6486: f64, t6490: f64, t6492: f64, t6495: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t6505 = t6504 * t67;
    let t6506 = t6505 * t1864;
    let t6509 = t71 * t641;
    let t6510 = t1863 * t6509;
    let t6514 = piecewise3(t8, 0.0_f64, -t6486 * t1865 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t6490 * t6492 + t6495 * t1865 / 3.0_f64 - t1860 * t6506 / 6.0_f64 - t1860 * t6510 / 6.0_f64);
    (t6505, t6506, t6509, t6510, t6514)
}
