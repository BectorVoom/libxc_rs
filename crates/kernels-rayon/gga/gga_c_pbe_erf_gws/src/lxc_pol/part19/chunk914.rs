//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 914/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk914(t10008: f64, t10010: f64, t10012: f64, t10015: f64, t10017: f64, t4503: f64, t4506: f64, t4513: f64, t4539: f64, t4542: f64, t4602: f64, t4744: f64, t6918: f64, t6932: f64, t7984: f64, t9764: f64) -> f64 {
    let t10247 = -t9764 - t6918 + t4503 - t4506 - t4513 + t4539 + t4542 + t10008 + t6932 + t10010 - t7984 + t10012 + t10015 + t10017 + t4602 + t4744;
    t10247
}
