//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2172/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2172(t3862: f64, t5231: f64, t12328: f64, t1815: f64, t1336: f64, t2691: f64, t3788: f64, t5252: f64, t3787: f64, t5318: f64, t40041: f64, t544: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t54785 = t5231 * t3862;
    let t54786 = 119.0_f64 / 4608.0_f64 * t54785;
    let t54793 = t1815 * t12328;
    let t54811 = t1336 * t3788 * t2691 * t5252;
    let t54812 = 119.0_f64 / 2304.0_f64 * t54811;
    let t54905 = t3787 * t5318;
    let t54963 = t544 * t68 * t40041;
    (t54786, t54793, t54812, t54905, t54963)
}
