//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 614/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk614(t27742: f64, t675: f64, t263: f64, t193: f64, t13927: f64, t6175: f64, t24412: f64, t3864: f64, t681: f64, t6843: f64, t1168: f64, t6187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27906 = t675 * t27742;
    let t27907 = t27906 * t263;
    let t27908 = t193 * t27907;
    let t27911 = t13927 * t6175;
    let t27913 = t24412 * t3864;
    let t27915 = t681 * t6843;
    let t27924 = t6187 * t1168;
    (t27906, t27908, t27911, t27913, t27915, t27924)
}
