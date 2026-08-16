//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 914/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk914(t4503: f64, t4506: f64, t4513: f64, t4539: f64, t4542: f64, t4744: f64, t6918: f64, t6932: f64, t6966: f64, t6969: f64, t7984: f64, t7985: f64, t7987: f64, t7989: f64, t7991: f64, t7992: f64) -> f64 {
    let t7993 = t6918 + t4503 - t4506 - t4513 + t4539 + t4542 + t6932 + t6966 + t6969 - t7984 - t7985 + t7987 - t7989 + t7991 + t7992 + t4744;
    t7993
}
