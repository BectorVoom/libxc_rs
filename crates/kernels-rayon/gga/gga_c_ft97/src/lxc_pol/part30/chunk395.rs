//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 395/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk395(t2843: f64, t6374: f64, t296: f64, t6315: f64, t6332: f64, t6312: f64, t6321: f64, t6325: f64, t6329: f64, t6337: f64, t6341: f64, t6345: f64) -> (f64, f64, f64, f64) {
    let t6375 = t2843 * t6374;
    let t6376 = t296 * t6375;
    let t6380 = t6315 / 6.0_f64;
    let t6383 = t6332 / 3.0_f64;
    let t6386 = t6312 / 4.0_f64 + t6380 + t6321 / 6.0_f64 + t6325 - t6329 / 2.0_f64 + t6383 + t6337 / 3.0_f64 + 2.0_f64 * t6341 - t6345;
    (t6376, t6380, t6383, t6386)
}
