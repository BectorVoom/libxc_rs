//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 338/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk338(t428: f64, t5522: f64, t397: f64, t52: f64, t67: f64, t11: f64, t391: f64, t41: f64) -> (f64, f64, f64) {
    let t5523 = t5522 * t428;
    let t5530 = t52 * t67 * t397;
    let t5532 = -0.1201569457037037037e0_f64 * t41 * t11 * t391 - 0.59273806478425129877e-2_f64 * t5530;
    (t5523, t5530, t5532)
}
