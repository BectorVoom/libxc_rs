//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1148/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1148(t12129: f64, t592: f64, t17: f64, t184: f64, t39454: f64, t1287: f64, t9216: f64, t2223: f64, t3826: f64, t11985: f64, t25: f64, t514: f64) -> (f64, f64, f64, f64, f64) {
    let t39851 = t592 * t12129;
    let t39852 = 48.0_f64 * t39851;
    let t39854 = t17 * t39454 * t184;
    let t39855 = t9216 * t1287;
    let t39856 = 960.0_f64 * t39855;
    let t39857 = t2223 * t3826;
    let t39858 = 384.0_f64 * t39857;
    let t39861 = 1.0_f64 / t514 / t11985 / t25;
    (t39852, t39854, t39856, t39858, t39861)
}
