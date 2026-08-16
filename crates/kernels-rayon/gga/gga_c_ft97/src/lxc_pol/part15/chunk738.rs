//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 738/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk738(t20781: f64, t20850: f64, t143: f64, t160: f64, t20224: f64, t3440: f64, t3439: f64, t13153: f64, t4823: f64, t12680: f64, t4828: f64, t17021: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20851 = t20781 + t20850;
    let t20853 = t143 * t20851 * t160;
    let t20858 = t3440 * t20224;
    let t20859 = t3439 * t20858;
    let t20862 = t13153 * t4823;
    let t20865 = t12680 * t4828;
    let t20868 = t17021 * t925;
    (t20851, t20853, t20858, t20859, t20862, t20865, t20868)
}
