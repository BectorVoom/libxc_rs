//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 549/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk549(t376: f64, t5509: f64, t1307: f64, t7241: f64, t5619: f64, t378: f64, t5507: f64) -> (f64, f64, f64, f64) {
    let t22878 = t376 * t5509;
    let t22883 = t7241 * t1307;
    let t22892 = t376 * t5619;
    let t22907 = t378 * t5507;
    (t22878, t22883, t22892, t22907)
}
