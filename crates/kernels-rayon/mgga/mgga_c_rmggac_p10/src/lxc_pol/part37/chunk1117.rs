//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1117/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1117(t14953: f64, t1562: f64, t14977: f64, t15865: f64, t4041: f64, t4985: f64, t71804: f64, t76103: f64, t76108: f64, t78486: f64, t78487: f64, t78488: f64, t78491: f64, t78493: f64, t78495: f64, t78497: f64, t78498: f64, t78499: f64, t78500: f64, t78501: f64) -> f64 {
    let t80517 = t1562 * t14953;
    let t80521 = -t78486 + t78487 + 0.59871208509319042821e-1_f64 * t4985 * t14977 - t71804 - t78488 - 0.58171619854173713844e-5_f64 * t76103 - 0.21814357445315142691e-4_f64 * t76108 - t78491 - 0.2363e1_f64 * t80517 + t78493 - t78495 - t78497 + 0.59871208509319042821e-1_f64 * t4041 * t15865 + t78498 + t78499 + t78500 + t78501;
    t80521
}
