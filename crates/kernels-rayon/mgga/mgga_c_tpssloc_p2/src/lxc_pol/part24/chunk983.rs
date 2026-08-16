//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 983/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk983(t154: f64, t3584: f64, t3241: f64, t636: f64, t9288: f64, t123: f64) -> (f64, f64, f64) {
    let t11145 = t154 * t3584;
    let t11147 = 1.0_f64 / t3241 / t636;
    let t11148 = t11147 * t9288;
    let t11149 = t11145 * t11148;
    let t11150 = t123 * t11149;
    (t11147, t11148, t11150)
}
