//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1001/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1001(t157: f64, t40465: f64, t8392: f64, t9425: f64, t24: f64, t32905: f64, t2159: f64, t8232: f64, t9094: f64, t9129: f64, t1882: f64, t9109: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40808 = t40465 * t157;
    let t40828 = t8392 * t9425;
    let t40830 = t24 * t32905;
    let t40835 = t8232 * t2159;
    let t40837 = t8392 * t9094;
    let t40840 = t8392 * t9129;
    let t40847 = t1882 * t9109;
    (t40808, t40828, t40830, t40835, t40837, t40840, t40847)
}
