//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 856/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk856(t13192: f64, t3206: f64, t4552: f64, t4992: f64, t86: f64, t13173: f64, t4555: f64, t3210: f64, t2816: f64, t5026: f64, t1092: f64, t2825: f64, t4995: f64) -> (f64, f64, f64, f64, f64) {
    let t13195 = t13192 * t3206;
    let t13199 = t86 * t4992 * t4552;
    let t13200 = t4555 * t13173;
    let t13201 = t3210 * t13200;
    let t13202 = t13199 * t13201;
    let t13204 = t5026 * t2816;
    let t13205 = t1092 * t13204;
    let t13207 = t2825 * t4995;
    (t13195, t13200, t13202, t13205, t13207)
}
