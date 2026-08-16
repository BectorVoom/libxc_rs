//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1544/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1544(t11177: f64, t300: f64, t1098: f64, t3256: f64, t1119: f64, t3259: f64, t3308: f64, t1094: f64, t3312: f64) -> (f64, f64, f64, f64, f64) {
    let t11179 = 0.19751673498613801407e-1_f64 * t300 * t11177;
    let t11180 = t3256 * t1098;
    let t11182 = 3.0_f64 * t11180 * t1119;
    let t11184 = 3.0_f64 * t3259 * t3308;
    let t11185 = t1094 * t3312;
    (t11179, t11180, t11182, t11184, t11185)
}
