//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2301/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2301(t193: f64, t2379: f64, t1484: f64, t2522: f64, t40622: f64, t4320: f64, t47166: f64, t47168: f64, t47171: f64, t47174: f64, t47175: f64, t47178: f64, t47181: f64, t47183: f64, t47186: f64) -> f64 {
    let t47645 = t193 * t2379;
    let t47651 = 3.0_f64 * t1484 * t2522 * t40622 + 18.0_f64 * t4320 * t47645 + t47166 + t47168 + t47171 + t47174 + t47175 + t47178 + t47181 + t47183 + t47186;
    t47651
}
