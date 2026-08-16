//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 214/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk214(t174: f64, t833: f64, t447: f64, t237: f64, t318: f64, t451: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t1299 = piecewise3(t175, 0.0_f64, t833);
    let t1300 = t447 * t1299;
    let t1305 = t237 * t318 * t451;
    (t1299, t1300, t1305)
}
