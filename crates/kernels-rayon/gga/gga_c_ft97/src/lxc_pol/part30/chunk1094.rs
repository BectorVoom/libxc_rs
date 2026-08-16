//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1094/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1094(t143158: f64, t152673: f64, t33820: f64, t143239: f64, t3886: f64, t2917: f64, t7641: f64, t143041: f64, t28736: f64, t33822: f64, t631: f64, t99475: f64) -> (f64, f64, f64, f64) {
    let t152675 = t33820 * t143158 * t152673;
    let t152678 = t143239 * t3886;
    let t152680 = t33820 * t2917 * t7641 * t152678;
    let t152686 = t99475 * t631 * t143041 * t33822 * t28736;
    (t152675, t152678, t152680, t152686)
}
