//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 990/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk990(t2131: f64, t2132: f64, t2217: f64, t847: f64, t7990: f64, t8065: f64, t32165: f64, t8104: f64, t32181: f64, t33232: f64, t4210: f64, t3035: f64, t3923: f64, t633: f64) -> (f64, f64, f64, f64, f64) {
    let t33278 = t2131 * t2132 * t2217 * t847;
    let t33281 = t7990 * t8065;
    let t33284 = 0.26020884564615598386e1_f64 * t32165 * t8104;
    let t33286 = t32181 * t33232 * t4210;
    let t33293 = 0.39512695097613069591e1_f64 * t3035 * t633 * t3923;
    (t33278, t33281, t33284, t33286, t33293)
}
