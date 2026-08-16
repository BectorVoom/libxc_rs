//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 687/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk687(t1060: f64, t574: f64, t5842: f64, t23571: f64, t3455: f64, t12968: f64, t13153: f64, t5856: f64, t6626: f64, t9419: f64, t23581: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26978 = t574 * t1060 * t5842;
    let t26981 = t23571 * t3455;
    let t26982 = t12968 * t26981;
    let t26985 = t13153 * t5856;
    let t26988 = t9419 * t6626;
    let t26991 = t23581 * t925;
    (t26978, t26981, t26982, t26985, t26988, t26991)
}
