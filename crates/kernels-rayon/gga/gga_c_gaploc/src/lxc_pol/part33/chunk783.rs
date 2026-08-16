//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 783/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk783(t7419: f64, t969: f64, t825: f64, t2685: f64, t2684: f64, t5638: f64, t60: f64) -> (f64, f64, f64) {
    let t7420 = t969 * t7419;
    let t7421 = t825 * t7420;
    let t7423 = t2685 * t7419;
    let t7424 = t2684 * t7423;
    let t7426 = t5638 * t60;
    (t7421, t7424, t7426)
}
