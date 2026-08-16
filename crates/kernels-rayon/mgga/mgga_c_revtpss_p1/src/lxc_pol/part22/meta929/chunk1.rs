//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3156/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3156(t17501: f64, t3172: f64, t3711: f64, t13099: f64, t43776: f64, t12956: f64, t17217: f64, t12909: f64, t17395: f64, t12784: f64, t17384: f64, t12772: f64, t17668: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57128 = t3711 * t3172 * t17501;
    let t57136 = t13099 * t43776;
    let t57145 = t12956 * t17217;
    let t57147 = t12909 * t17395;
    let t57164 = t12784 * t17384;
    let t57167 = t3625 * t12772 * t17668;
    (t57128, t57136, t57145, t57147, t57164, t57167)
}
