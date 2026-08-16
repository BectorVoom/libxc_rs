//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2389/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2389(t39706: f64, t39749: f64, t39803: f64, t39840: f64, t17: f64, t521: f64, t2225: f64, t3826: f64, t12129: f64, t592: f64, t2223: f64, t11985: f64, t25: f64, t514: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39842 = t39706 + t39749 + t39803 + t39840;
    let t39844 = t17 * t521 * t39842;
    let t39845 = t2225 * t3826;
    let t39851 = t592 * t12129;
    let t39857 = t2223 * t3826;
    let t39861 = 1.0_f64 / t514 / t11985 / t25;
    (t39842, t39844, t39845, t39851, t39857, t39861)
}
