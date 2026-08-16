//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 415/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk415(t193: f64, t6109: f64, t6879: f64, t1091: f64, t2354: f64, t6119: f64, t6118: f64, t2506: f64, t6852: f64, t1434: f64, t6837: f64, t743: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6881 = t6109 * t193 * t6879;
    let t6884 = t2354 * t6119 * t1091;
    let t6885 = t6118 * t6884;
    let t6887 = t2506 * t6852;
    let t6889 = t1434 * t193 * t6887;
    let t6891 = t743 * t6837;
    (t6881, t6884, t6885, t6887, t6889, t6891)
}
