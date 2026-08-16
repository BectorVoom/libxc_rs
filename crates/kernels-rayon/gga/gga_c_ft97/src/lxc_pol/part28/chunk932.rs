//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 932/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk932(t22878: f64, t7162: f64, t1586: f64, t32325: f64, t22914: f64, t32021: f64, t32076: f64, t72: f64, t7243: f64) -> (f64, f64, f64, f64, f64) {
    let t136098 = t7162 * t22878;
    let t136116 = t1586 * t32325;
    let t136121 = t22914 * t32021;
    let t136138 = t72 * t32076;
    let t136151 = t72 * t7243;
    (t136098, t136116, t136121, t136138, t136151)
}
