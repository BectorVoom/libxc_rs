//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2829/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2829(t11273: f64, t11998: f64, t1062: f64, t11782: f64, t11853: f64, t828: f64, t3229: f64, t360: f64, t3089: f64, t1087: f64, t1024: f64, t12003: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42371 = t11273 * t11998;
    let t42391 = t11782 * t1062;
    let t42410 = t828 * t11853;
    let t42415 = t360 * t3229;
    let t42416 = t42415 * t3089;
    let t42417 = t1087 * t42416;
    let t42425 = t1024 * t12003;
    (t42371, t42391, t42410, t42416, t42417, t42425)
}
