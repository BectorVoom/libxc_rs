//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 934/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk934(t2131: f64, t2147: f64, t463: f64, t7997: f64, t157: f64, t406: f64, t847: f64, t7932: f64, t7963: f64, t2132: f64, t2139: f64, t7885: f64, t879: f64) -> (f64, f64, f64) {
    let t32187 = t2131 * t2147 * t7997 * t463;
    let t32194 = t847 * t406 * t157;
    let t32196 = t7963 * t7932 * t32194;
    let t32210 = 0.78062653693846795158e1_f64 * t7885 * t2132 * t2139 * t879;
    (t32187, t32196, t32210)
}
