//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1359/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1359(t63935: f64, t63945: f64, t63949: f64, t63957: f64, t63964: f64, t66420: f64, t69972: f64, t69974: f64, t69976: f64, t69978: f64, t69981: f64, t69983: f64, t69985: f64) -> f64 {
    let t72069 = -t69972 / 24.0_f64 + t69974 / 96.0_f64 + t69976 / 96.0_f64 - t69978 / 96.0_f64 - t63935 - 7.0_f64 / 24.0_f64 * t69981 + 7.0_f64 / 72.0_f64 * t69983 + t69985 / 192.0_f64 - 119.0_f64 / 1728.0_f64 * t63945 - t63949 - 35.0_f64 / 54.0_f64 * t63957 + t66420 - 119.0_f64 / 432.0_f64 * t63964;
    t72069
}
