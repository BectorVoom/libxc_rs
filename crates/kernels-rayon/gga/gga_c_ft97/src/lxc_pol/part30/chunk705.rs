//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 705/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk705(t7124: f64, t824: f64, t840: f64, t871: f64, t28855: f64, t296: f64, t28931: f64, t24890: f64, t4256: f64, t312: f64, t7021: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29245 = t7124 * t824;
    let t29247 = t840 * t871 * t29245;
    let t29250 = t296 * t28855;
    let t29253 = t296 * t28931;
    let t29256 = t24890 * t4256;
    let t29259 = t312 * t7021;
    let t29260 = t29259 * t684;
    (t29245, t29247, t29250, t29253, t29256, t29260)
}
