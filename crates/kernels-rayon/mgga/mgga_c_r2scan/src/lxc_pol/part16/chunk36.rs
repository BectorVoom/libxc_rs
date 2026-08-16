//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 36/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk36(t41: f64, t89: f64, t61: f64, t86: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t90 = t41 * t89;
    let t92 = 0.19751673498613801407e-1_f64 * t61 * t86;
    let t93 = f64::ln(2.0_f64);
    let t94 = 1.0_f64 - t93;
    let t95 = pi * pi;
    (t90, t92, t94, t95)
}
