//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2643/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2643(t1336: f64, t2691: f64, t3788: f64, t5252: f64, t16028: f64, t225: f64, t40041: f64, t544: f64, t68: f64, t1332: f64, t16046: f64, t1338: f64, t16413: f64) -> (f64, f64, f64, f64, f64) {
    let t54811 = t1336 * t3788 * t2691 * t5252;
    let t54825 = t16028 * t225;
    let t54963 = t544 * t68 * t40041;
    let t54976 = t1332 * t16046;
    let t55039 = t1338 * t16413;
    (t54811, t54825, t54963, t54976, t55039)
}
