//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 927/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk927<F: Float>(t7998: F, t8397: F, t1659: F, t7973: F, t1539: F, t309: F, t32181: F, t36433: F, t463: F, t32003: F, t157: F, t32130: F, t32029: F, t557: F, t2934: F, t609: F) -> (F, F, F, F, F, F, F) {
    let t36460 = t8397 * t7998;
    let t36473 = 0.13170898365871023197e1 * t7973 * t1659;
    let t36475 = t1539 * t309;
    let t36477 = t32181 * t36433 * t36475;
    let t36479 = t1539 * t463;
    let t36482 = 0.34694512752820797848e1 * t32003 * t36433 * t36479;
    let t36495 = t157 * t463 * t309;
    let t36498 = 0.34694512752820797848e1 * t32130 * t36433 * t36495;
    let t36504 = t32029 * t557;
    let t36515 = t2934 * t609;
    (t36460, t36473, t36477, t36482, t36498, t36504, t36515)
}
