//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 771/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk771(t1969: f64, t8516: f64, t7229: f64, t7243: f64, t2134: f64, t27: f64, t3118: f64, t321: f64, t504: f64, t7262: f64, t507: f64, t7191: f64) -> (f64, f64, f64, f64, f64) {
    let t36336 = t8516 * t1969;
    let t36343 = t7229 * t7243;
    let t36402 = t2134 * t27 * t3118 * t321;
    let t36457 = t504 * t7262;
    let t36471 = t507 * t7191;
    (t36336, t36343, t36402, t36457, t36471)
}
