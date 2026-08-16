//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1595/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1595(t17191: f64, t324: f64, t300: f64, t5689: f64, t892: f64, t914: f64, t11094: f64, t5950: f64, t3216: f64, t5946: f64, t4483: f64, t4498: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17192 = t17191 * t324;
    let t17194 = 0.19751673498613801407e-1_f64 * t300 * t17192;
    let t17195 = t5689 * t892;
    let t17197 = 1.0_f64 * t17195 * t914;
    let t17198 = t5950 * t11094;
    let t17202 = t5946 * t3216;
    let t17209 = 0.34631718211362927517e2_f64 * t4483 * t4498;
    (t17192, t17194, t17195, t17197, t17198, t17202, t17209)
}
