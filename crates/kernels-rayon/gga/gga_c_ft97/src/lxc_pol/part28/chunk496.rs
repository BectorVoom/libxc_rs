//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 496/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk496(t1702: f64, t52: f64, t12: f64, t1593: f64, t1609: f64, t39: f64, t409: f64, t64: f64) -> (f64, f64, f64, f64, f64) {
    let t7839 = t52 * t1702;
    let t7853 = t52 * t12;
    let t7857 = t1609 * t1593;
    let t7866 = t409 * t39;
    let t7867 = t64 * t7866;
    (t7839, t7853, t7857, t7866, t7867)
}
