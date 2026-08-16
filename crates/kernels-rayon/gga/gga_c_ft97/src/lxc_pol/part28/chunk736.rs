//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 736/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk736(t22817: f64, t40: f64, t1669: f64, t11: f64, t58: f64, t171: f64) -> (f64, f64, f64, f64) {
    let t32137 = t22817 * t40;
    let t32138 = t1669 * t32137;
    let t32139 = t11 * t58;
    let t32140 = t32139 * t171;
    (t32137, t32138, t32139, t32140)
}
