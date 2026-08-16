//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 400/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk400(t6008: f64, t6752: f64, t193: f64, t1095: f64, t679: f64, t200: f64, t6014: f64, t1113: f64, t203: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6753 = t6008 * t6752;
    let t6754 = t193 * t6753;
    let t6757 = t679 * t1095;
    let t6758 = t6757 * t200;
    let t6759 = t6014 * t6758;
    let t6762 = t203 * t1113;
    (t6753, t6754, t6757, t6758, t6759, t6762)
}
