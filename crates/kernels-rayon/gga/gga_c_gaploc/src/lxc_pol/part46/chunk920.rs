//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 920/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk920(t2325: f64, t31501: f64, t882: f64, t883: f64, t2268: f64, t3158: f64, t8195: f64, t8199: f64, t9181: f64, t2321: f64, t34604: f64, t9074: f64) -> (f64, f64, f64, f64) {
    let t42889 = t882 * t2325 * t883 * t31501;
    let t42893 = 0.42682509953514224398e0_f64 * t2268 * t3158 * t8195;
    let t42896 = 0.14227503317838074799e1_f64 * t2268 * t9181 * t8199;
    let t42898 = t9074 * t34604 * t2321;
    (t42889, t42893, t42896, t42898)
}
