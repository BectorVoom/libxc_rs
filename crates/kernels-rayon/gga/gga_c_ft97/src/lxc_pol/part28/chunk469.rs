//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 469/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk469(t1852: f64, t7274: f64, t83: f64, t7246: f64, t7250: f64, t7254: f64, t7258: f64, t7262: f64) -> (f64, f64, f64) {
    let t7275 = t1852 * t7274;
    let t7276 = t83 * t7275;
    let t7281 = -t7246 + t7250 - t7254 / 2.0_f64 + 2.0_f64 * t7258 - t7262;
    (t7275, t7276, t7281)
}
