//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 997/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk997(t1882: f64, t33147: f64, t33133: f64, t1637: f64, t7392: f64, t89: f64, t33024: f64, t33142: f64, t33020: f64, t33151: f64, t33121: f64, t376: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t140376 = t1882 * t33147;
    let t140378 = t1882 * t33133;
    let t140382 = 4.0_f64 / 27.0_f64 * t89 * t1637 * t7392;
    let t140383 = t1882 * t33024;
    let t140390 = t1882 * t33142;
    let t140395 = t1882 * t33020;
    let t140397 = t1882 * t33151;
    let t140412 = t89 * t376 * t33121;
    (t140376, t140378, t140382, t140383, t140390, t140395, t140397, t140412)
}
