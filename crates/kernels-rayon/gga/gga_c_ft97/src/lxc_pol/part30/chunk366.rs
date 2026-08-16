//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 366/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk366(t6166: f64, t729: f64, t762: f64, t1449: f64, t2469: f64, t242: f64, t766: f64) -> (f64, f64, f64) {
    let t6168 = t729 * t762 * t6166;
    let t6171 = t2469 * t1449;
    let t6172 = t242 * t6171;
    let t6175 = t1449 * t766;
    (t6168, t6172, t6175)
}
