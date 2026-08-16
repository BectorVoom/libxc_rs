//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 344/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk344(t1000: f64, t1005: f64, t1020: f64, t1025: f64, t1032: f64, t1038: f64, t1041: f64, t1046: f64, t350: f64, t378: f64, t964: f64, t973: f64, t997: f64) -> f64 {
    let t1049 = -t964 * t350 / 36.0_f64 + t997 + t973 * t1000 / 288.0_f64 + t1005 * t378 / 3072.0_f64 + t1020 * t1025 / 3072.0_f64 - t1032 * t378 / 576.0_f64 + t1038 + t1041 * t1046 / 4608.0_f64;
    t1049
}
