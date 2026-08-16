//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1133/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1133(t35972: f64, t668: f64, t2665: f64, t446: f64, t505: f64, t143187: f64, t143204: f64, t143245: f64, t143264: f64, t152948: f64, t152952: f64, t152954: f64, t152958: f64, t152962: f64, t152965: f64, t152970: f64, t152975: f64, t152979: f64, t153375: f64, t153379: f64) -> (f64, f64) {
    let t153381 = t35972 * t668;
    let t153384 = t446 * t2665 * t153381 * t505;
    let t153386 = 8.0_f64 * t152948 - 4.0_f64 * t152952 + 2.0_f64 / 27.0_f64 * t152954 - 2.0_f64 / 9.0_f64 * t152958 + t143187 / 18.0_f64 - 8.0_f64 / 9.0_f64 * t152962 - 4.0_f64 / 9.0_f64 * t152965 - 2.0_f64 / 9.0_f64 * t143204 - t143245 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t152970 + 2.0_f64 / 3.0_f64 * t152975 + 2.0_f64 / 3.0_f64 * t152979 + t143264 - t153375 / 6.0_f64 - 4.0_f64 * t153379 + t153384 / 9.0_f64;
    (t153384, t153386)
}
