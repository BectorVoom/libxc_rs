//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 888/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk888(t24886: f64, t7101: f64, t1091: f64, t34081: f64, t10492: f64, t296: f64, t36043: f64, t1255: f64, t7611: f64, t840: f64, t35817: f64, t1476: f64, t7131: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36161 = t24886 * t7101;
    let t36164 = t34081 * t1091;
    let t36165 = t10492 * t36164;
    let t36168 = t296 * t36043;
    let t36172 = t840 * t1255 * t7611;
    let t36175 = t296 * t35817;
    let t36179 = t840 * t7131 * t1476;
    (t36161, t36164, t36165, t36168, t36172, t36175, t36179)
}
