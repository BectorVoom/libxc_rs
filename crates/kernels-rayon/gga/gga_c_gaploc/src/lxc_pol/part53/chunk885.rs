//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 885/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk885(t2321: f64, t34604: f64, t9074: f64, t29650: f64, t2972: f64, t13235: f64, t14537: f64, t8862: f64, t9784: f64, t3073: f64, t9767: f64, t13200: f64, t29439: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42898 = t9074 * t34604 * t2321;
    let t42899 = 0.23712505529730124666e-2_f64 * t42898;
    let t42906 = 2.0_f64 * t29650 * t2972;
    let t42908 = 6.0_f64 * t14537 * t13235;
    let t42910 = 2.0_f64 * t8862 * t9784;
    let t42916 = t9767 * t3073;
    let t42933 = t29439 * t13200;
    (t42899, t42906, t42908, t42910, t42916, t42933)
}
