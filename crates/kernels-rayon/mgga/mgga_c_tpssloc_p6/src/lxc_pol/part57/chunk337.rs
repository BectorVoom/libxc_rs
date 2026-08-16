//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 337/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk337(t1983: f64, t2020: f64, t113: f64, t1869: f64, t1876: f64, t1976: f64, t1980: f64, t510: f64, t574: f64) -> f64 {
    let t2021 = t1983 * t2020;
    let t2022 = -t113 * t1976 - t1869 * t510 + t1980 * t574 - t1876 + t2021;
    t2022
}
