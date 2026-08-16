//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 790/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk790(t192: f64, t21416: f64, t9942: f64, t2372: f64, t3930: f64, t5053: f64, t21181: f64, t9953: f64, t9952: f64, t2487: f64, t737: f64, t21204: f64, t3917: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21577 = t192 * t9942 * t21416;
    let t21581 = t2372 * t3930 * t5053;
    let t21584 = t9953 * t21181;
    let t21585 = t9952 * t21584;
    let t21588 = t2487 * t21181;
    let t21589 = t737 * t21588;
    let t21592 = t3917 * t21204;
    (t21577, t21581, t21584, t21585, t21588, t21589, t21592)
}
