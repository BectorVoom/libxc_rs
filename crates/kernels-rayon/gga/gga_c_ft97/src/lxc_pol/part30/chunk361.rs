//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 361/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk361(t2354: f64, t6119: f64, t684: f64, t6118: f64, t2506: f64, t6079: f64, t1434: f64, t193: f64, t6061: f64, t743: f64, t1439: f64, t375: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6121 = t2354 * t6119 * t684;
    let t6122 = t6118 * t6121;
    let t6124 = t2506 * t6079;
    let t6126 = t1434 * t193 * t6124;
    let t6128 = t743 * t6061;
    let t6130 = t1434 * t193 * t6128;
    let t6133 = t89 * t375 * t1439;
    (t6121, t6122, t6124, t6126, t6128, t6130, t6133)
}
