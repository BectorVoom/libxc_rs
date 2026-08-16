//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1321/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1321(t14185: f64, t17964: f64, t14304: f64, t5547: f64, t14229: f64, t14234: f64, t14176: f64, t19703: f64, t4708: f64, t61072: f64, t17946: f64, t4712: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69968 = t17964 * t14185;
    let t69972 = t5547 * t14304;
    let t69974 = t17964 * t14229;
    let t69976 = t17964 * t14234;
    let t69978 = t19703 * t14176;
    let t69981 = t61072 * t4708;
    let t69983 = t17946 * t4712;
    (t69968, t69972, t69974, t69976, t69978, t69981, t69983)
}
