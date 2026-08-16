//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 756/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk756(t104: f64, t2248: f64, t3984: f64, t8040: f64, t2217: f64, t322: f64, t2132: f64, t2138: f64, t633: f64, t879: f64, t2147: f64, t463: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8048 = t104 * t2248;
    let t8052 = t8040 * t3984;
    let t8060 = t2217 * t322;
    let t8061 = t2132 * t8060;
    let t8062 = t2138 * t8061;
    let t8064 = t633 * t879;
    let t8065 = t2132 * t8064;
    let t8067 = 0.8673628188205199462e0_f64 * t2138 * t8065;
    let t8069 = t2147 * t2217 * t463;
    (t8048, t8052, t8060, t8061, t8062, t8064, t8065, t8067, t8069)
}
