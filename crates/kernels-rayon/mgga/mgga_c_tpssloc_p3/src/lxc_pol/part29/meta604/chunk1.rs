//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2041/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2041(t23165: f64, t82038: f64, t1879: f64, t80845: f64, t1906: f64, t23229: f64, t81715: f64, t225: f64, t23226: f64, t23228: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82039 = t82038 * t23165;
    let t82045 = t80845 * t1879;
    let t82046 = t82045 * t1906;
    let t82047 = 0.27720185200590482541e0_f64 * t82046;
    let t82069 = t81715 * t23229;
    let t82070 = 0.98696044010893586188e-1_f64 * t82069;
    let t82071 = t23226 * t225;
    let t82074 = t23228 * t225;
    (t82039, t82045, t82047, t82070, t82071, t82074)
}
