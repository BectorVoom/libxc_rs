//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 548/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk548(t51: f64, t3010: f64, t476: f64, t3008: f64, t3006: f64, zeta_threshold: f64) -> (f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t3011 = t476 * t3010;
    let t3014 = piecewise3(t52, 0.0_f64, -2.0_f64 / 9.0_f64 * t3008 + 2.0_f64 / 3.0_f64 * t3011);
    let t3016 = t3006 / 2.0_f64 + t3014 / 2.0_f64;
    (t3011, t3016)
}
