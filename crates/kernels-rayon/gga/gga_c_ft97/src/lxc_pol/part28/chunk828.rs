//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 828/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk828(t144: f64, t32995: f64, t167: f64, t32869: f64, t574: f64, t1882: f64, t7409: f64, t376: f64, t7392: f64, t89: f64, t1901: f64, t33176: f64, t33180: f64, t33184: f64, t33188: f64, t33193: f64, t33196: f64, t33200: f64, t33204: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t33207 = t144 * t32995;
    let t33211 = t574 * t167 * t32869;
    let t33215 = t1882 * t7409 / 9.0_f64;
    let t33218 = t89 * t376 * t7392 / 9.0_f64;
    let t33219 = 4.0_f64 / 3.0_f64 * t446 * t33176 + 4.0_f64 / 3.0_f64 * t446 * t33180 - t446 * t33184 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t33188 - 2.0_f64 / 9.0_f64 * t1901 * t33193 + 2.0_f64 / 9.0_f64 * t1901 * t33196 - 4.0_f64 / 3.0_f64 * t1901 * t33200 - 4.0_f64 / 3.0_f64 * t1901 * t33204 + 2.0_f64 / 3.0_f64 * t446 * t33207 - t446 * t33211 / 3.0_f64 + t33215 - t33218;
    (t33207, t33211, t33215, t33218, t33219)
}
