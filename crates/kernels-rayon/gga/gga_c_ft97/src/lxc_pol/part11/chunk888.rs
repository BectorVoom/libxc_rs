//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 888/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk888(t79: f64, t37522: f64, t37607: f64, t37666: f64, t37883: f64, t37957: f64, t38035: f64, t38175: f64, t38250: f64, t27: f64, t370: f64, t89: f64, t1636: f64, t1756: f64) -> (f64, f64, f64) {
    let t80 = 0.1e-59_f64 < t79;
    let t38254 = piecewise3(t80, t37522 + t37607 + t37666 + t37883 + t37957 + t38035 + t38175 + t38250, 0.0_f64);
    let t38257 = t89 * t27 * t370 * t38254;
    let t38260 = t89 * t1636 * t1756;
    (t38254, t38257, t38260)
}
