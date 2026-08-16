//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 864/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk864(t13208: f64, t16671: f64, t13212: f64, t16675: f64, t1882: f64, t4833: f64, t4747: f64, t4743: f64, t4735: f64, t4739: f64, t376: f64, t4792: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17426 = t13208 * t16671;
    let t17429 = t13212 * t16675;
    let t17432 = t1882 * t4833;
    let t17434 = t1882 * t4747;
    let t17436 = t1882 * t4743;
    let t17438 = t1882 * t4735;
    let t17440 = t1882 * t4739;
    let t17443 = t89 * t376 * t4792;
    (t17426, t17429, t17432, t17434, t17436, t17438, t17440, t17443)
}
