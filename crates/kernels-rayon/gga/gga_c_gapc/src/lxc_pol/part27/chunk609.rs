//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 609/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk609(t3664: f64, t3665: f64, t122: f64, t515: f64, t125: f64, t169: f64) -> (f64, f64, f64, f64) {
    let t3666 = t3664 * t3665;
    let t3668 = t515 * t122;
    let t3669 = t3668 * t125;
    let t3670 = t169 * t3669;
    (t3666, t3668, t3669, t3670)
}
